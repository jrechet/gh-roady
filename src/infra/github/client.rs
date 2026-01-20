use async_trait::async_trait;
use octocrab::{Octocrab, Page};
use crate::domain::artifact::Artifact;
use crate::domain::repo::Repository;
use crate::domain::user::User;
use crate::domain::github::GithubRepository;
use crate::domain::error::Result;
use crate::domain::storage::StorageUsageReport;

pub struct GitHubClient {
    client: Octocrab,
}

impl GitHubClient {
    pub fn new(token: String) -> Result<Self> {
        let client = Octocrab::builder()
            .personal_token(token)
            .build()?;
        
        Ok(Self { client })
    }
}

#[async_trait]
impl GithubRepository for GitHubClient {
    async fn get_current_user(&self) -> Result<User> {
        let user_profile = self.client.current().user().await?;
        Ok(User::from(user_profile))
    }

    async fn list_user_repos(&self) -> Result<Vec<Repository>> {
        let mut repos = Vec::new();
        let mut page_num = 1;
        loop {
            let page: Page<octocrab::models::Repository> = self
                .client
                .current()
                .list_repos_for_authenticated_user()
                .per_page(100)
                .page(page_num as u8)
                .send()
                .await?;

            let len = page.items.len();
            repos.extend(page.items.into_iter().map(Repository::from));

            if len < 100 {
                break;
            }
            page_num += 1;
        }

        Ok(repos)
    }

    async fn list_repos_for(&self, owner: &str) -> Result<Vec<Repository>> {
        let mut repos = Vec::new();
        let mut page_num = 1;
        
        loop {
            let result = self.client.users(owner).repos().per_page(100).page(page_num as u32).send().await;
            match result {
                Ok(page) => {
                    let len = page.items.len();
                    repos.extend(page.items.into_iter().map(Repository::from));
                    if len < 100 { break; }
                }
                Err(_) => {
                    // Try as org
                    let result = self
                        .client
                        .orgs(owner)
                        .list_repos()
                        .per_page(100)
                        .page(page_num as u32)
                        .send()
                        .await;
                    
                    if let Ok(page) = result {
                        let len = page.items.len();
                        repos.extend(page.items.into_iter().map(Repository::from));
                        if len < 100 { break; }
                    } else {
                        break;
                    }
                }
            }
            page_num += 1;
        }
        Ok(repos)
    }

    async fn get_repo(&self, owner: &str, repo: &str) -> Result<Repository> {
        let r = self.client.repos(owner, repo).get().await?;
        Ok(Repository::from(r))
    }

    async fn list_artifacts_for_repo(&self, owner: &str, repo: &str) -> Result<Vec<Artifact>> {
        let route = format!("/repos/{}/{}/actions/artifacts", owner, repo);
        let page: Page<octocrab::models::workflows::WorkflowListArtifact> = self.client.get(route, None::<&()>).await?;
        
        let mut artifacts: Vec<Artifact> = page.items.into_iter().map(Artifact::from).collect();
        for a in &mut artifacts {
            a.repository_owner = owner.to_string();
            a.repository_name = repo.to_string();
        }
        
        Ok(artifacts)
    }

    async fn list_all_artifacts(&self) -> Result<Vec<Artifact>> {
        let repos = self.list_user_repos().await?;
        let mut all_artifacts = Vec::new();
        
        for r in repos {
            if let Ok(artifacts) = self.list_artifacts_for_repo(&r.owner, &r.name).await {
                all_artifacts.extend(artifacts);
            }
        }
        
        Ok(all_artifacts)
    }

    async fn delete_artifact(&self, owner: &str, repo: &str, artifact_id: u64) -> Result<()> {
        let route = format!("/repos/{owner}/{repo}/actions/artifacts/{artifact_id}");
        // We use _delete and map_github_error because the response is 204 No Content,
        // which causes an EOF error in the generic delete method that expects JSON.
        let resp = self.client._delete(route, None::<&()>).await?;
        octocrab::map_github_error(resp).await?;
        Ok(())
    }

    async fn delete_cache(&self, owner: &str, repo: &str, cache_id: u64) -> Result<()> {
        let route = format!("/repos/{owner}/{repo}/actions/caches/{cache_id}");
        let resp = self.client._delete(route, None::<&()>).await?;
        octocrab::map_github_error(resp).await?;
        Ok(())
    }

    async fn get_storage_usage(&self) -> Result<StorageUsageReport> {
        let mut total_used_billing = 0;
        // Default to 2GB (Pro) as it's safer to overestimate, or 500MB if we can confirm Free
        let mut total_max: u64 = 2 * 1024 * 1024 * 1024; 
        let mut items = Vec::new();

        // 1. Check Personal Billing
        let personal_billing_route = "/user/billing/shared-storage";
        if let Ok(resp) = self.client.get::<serde_json::Value, _, _>(personal_billing_route, None::<&()>).await {
            total_used_billing = resp.get("estimated_storage_for_month")
                .and_then(|v| v.as_u64())
                .map(|gb| gb * 1024 * 1024 * 1024)
                .unwrap_or(0);
            
            if let Some(included) = resp.get("included_gigabytes_bandwidth_used").and_then(|v| v.as_u64()) {
                if included == 0 {
                    // Likely Free plan if 0 included GB
                    total_max = 500 * 1024 * 1024;
                }
            }
        }

        // 2. Determine Plan and Max from /user endpoint (if available)
        if let Ok(user_val) = self.client.get::<serde_json::Value, _, _>("/user", None::<&()>).await {
            if let Some(plan) = user_val.get("plan") {
                if let Some(name) = plan.get("name").and_then(|n| n.as_str()) {
                    let plan_name = name.to_lowercase();
                    total_max = match plan_name.as_str() {
                        "free" => 500 * 1024 * 1024,
                        "pro" => 2 * 1024 * 1024 * 1024,
                        "team" => 2 * 1024 * 1024 * 1024,
                        "enterprise" | "enterprise_cloud" => 50 * 1024 * 1024 * 1024,
                        _ => 2 * 1024 * 1024 * 1024, // Default to Pro/2GB for unknown
                    };
                }
            }
        }

        // 3. Check Organizations Billing (for total_used)
        if let Ok(orgs_val) = self.client.get::<serde_json::Value, _, _>("/user/orgs", None::<&()>).await {
            if let Some(orgs_array) = orgs_val.as_array() {
                for org_obj in orgs_array {
                    if let Some(org_login) = org_obj.get("login").and_then(|l| l.as_str()) {
                        let org_billing_route = format!("/orgs/{}/billing/shared-storage", org_login);
                        if let Ok(resp) = self.client.get::<serde_json::Value, _, _>(org_billing_route, None::<&()>).await {
                            total_used_billing += resp.get("total_usage_in_bytes")
                                .and_then(|v| v.as_u64())
                                .unwrap_or(0);
                        }
                    }
                }
            }
        }

        // 4. Scan all repos for details
        if let Ok(repos) = self.list_user_repos().await {
            use crate::domain::storage::{StorageItem, StorageItemType};
            for r in repos {
                // Fetch Artifacts
                if let Ok(artifacts) = self.list_artifacts_for_repo(&r.owner, &r.name).await {
                    for a in artifacts {
                        items.push(StorageItem {
                            id: a.id,
                            name: format!("Artifact: {}", a.name),
                            owner: r.owner.clone(),
                            repo: r.name.clone(),
                            size_in_bytes: a.size_in_bytes,
                            item_type: StorageItemType::Artifact,
                        });
                    }
                }
                
                // Fetch Caches
                let cache_route = format!("/repos/{}/{}/actions/caches", r.owner, r.name);
                if let Ok(cache_resp) = self.client.get::<serde_json::Value, _, _>(cache_route, None::<&()>).await {
                    if let Some(caches) = cache_resp.get("actions_caches").and_then(|c| c.as_array()) {
                        for c in caches {
                            if let (Some(id), Some(key)) = (c.get("id").and_then(|i| i.as_u64()), c.get("key").and_then(|k| k.as_str())) {
                                items.push(StorageItem {
                                    id,
                                    name: format!("Cache: {}", key),
                                    owner: r.owner.clone(),
                                    repo: r.name.clone(),
                                    size_in_bytes: c.get("size_in_bytes").and_then(|s| s.as_u64()).unwrap_or(0),
                                    item_type: StorageItemType::Cache,
                                });
                            }
                        }
                    }
                }
            }
        }

        // If billing returned 0 but we found stuff, or vice versa, we use the max of both
        // Actually the billing might include Packages which we don't list yet.
        let scanned_total: u64 = items.iter().map(|i| i.size_in_bytes).sum();
        let total_used = std::cmp::max(total_used_billing, scanned_total);

        Ok(StorageUsageReport {
            total_used,
            max_allowed: total_max,
            items,
        })
    }
}
