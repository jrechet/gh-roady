use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum StorageItemType {
    Artifact,
    Cache,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StorageItem {
    pub id: u64,
    pub name: String,
    pub owner: String,
    pub repo: String,
    pub size_in_bytes: u64,
    pub item_type: StorageItemType,
}

pub struct StorageUsageReport {
    pub total_used: u64,
    pub max_allowed: u64,
    pub items: Vec<StorageItem>,
}
