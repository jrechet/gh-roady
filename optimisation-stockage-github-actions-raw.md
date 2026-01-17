# Gestion et Optimisation du Stockage GitHub Actions avec `ghr`

Cet article détaille la mise en place et l'utilisation de l'outil `ghr` (gh-roady), conçu pour monitorer et nettoyer efficacement le stockage partagé de vos GitHub Actions (artéfacts et caches).

## 1. Prérequis

Avant de commencer, assurez-vous d'avoir installé les outils suivants :

*   **Rust (compilateur et cargo)** : `curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh`
*   **Un Personal Access Token (PAT) GitHub** avec les permissions `repo` et `actions`.
    *   [Documentation GitHub : Créer un PAT](https://docs.github.com/en/authentication/keeping-your-account-and-data-secure/managing-your-personal-access-tokens)

## 2. Installation de l'outil `ghr`

Vous pouvez soit télécharger les binaires pré-compilés (Linux, macOS, Windows) depuis la [page des Releases](https://github.com/jrechet/gh-roady/releases), soit compiler le projet manuellement.

```bash
# Clonez le repo
git clone https://github.com/jrechet/gh-roady.git
cd gh-roady

# Compilation
cargo build --release

# Vérification : vérifier que l'exécutable est bien généré
ls -lh ./target/release/ghr
```

## 3. Configuration de l'authentification

L'outil nécessite une authentification pour communiquer avec l'API GitHub. Vous pouvez utiliser une variable d'environnement ou le fichier `.env`.

```bash
# Création du fichier d'environnement
echo "GITHUB_TOKEN=votre_token_ici" > .env

# Login via la commande (optionnel si le .env est présent)
./target/release/ghr auth login --token votre_token_ici

# Vérification : tester la connexion
./target/release/ghr artifacts list --limit 1
```

## 4. Monitoring du stockage (`df`)

La commande `df` permet d'obtenir un état des lieux précis du stockage consommé par rapport à la limite de votre forfait GitHub (Free, Pro, Team, Enterprise).

```bash
# Analyse du stockage
./target/release/ghr df
```
*Cette commande scanne vos artéfacts et vos caches sur l'ensemble de vos dépôts et de vos organisations.*

## 5. Nettoyage interactif

Si le stockage est saturé, `ghr df` propose une interface interactive pour sélectionner les éléments à supprimer.

1.  Exécutez `./target/release/ghr df`.
2.  Utilisez la **barre d'espace** pour sélectionner les caches ou artéfacts volumineux.
3.  Appuyez sur **Entrée** pour lancer la suppression.

```bash
# Vérification après nettoyage
./target/release/ghr df
```
*Le pourcentage d'utilisation devrait avoir diminué.*

---
*Sujet traité : Création d'un outil CLI Rust pour la gestion fine du stockage GitHub Actions.*
