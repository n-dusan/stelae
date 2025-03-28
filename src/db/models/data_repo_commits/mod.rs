use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use sqlx::{any::AnyRow, FromRow, Row as _};

pub mod manager;

/// Trait for managing transactional data repo commits.
#[async_trait]
pub trait TxManager {
    /// Find all authentication commits for a given stele.
    async fn find_all_auth_commits_for_stele(
        &mut self,
        stele_id: &str,
    ) -> anyhow::Result<Vec<DataRepoCommits>>;
    /// Insert a bulk of data repo commits.
    async fn insert_bulk(&mut self, data_repo_commits: Vec<DataRepoCommits>) -> anyhow::Result<()>;
}

#[derive(Debug, Deserialize, Serialize)]
/// Model for the commits within the data repository.
pub struct DataRepoCommits {
    /// Unique commit hash of the authentication repository.
    pub commit_hash: String,
    /// Codified date.
    pub codified_date: Option<String>,
    /// Build date of the commit.
    pub build_date: Option<String>,
    /// Type of the data repository. E.g. `html`.
    pub repo_type: String,
    /// Foreign key reference to the authentication commit hash.
    pub auth_commit_hash: String,
    /// Timestamp of the authentication commit.
    pub auth_commit_timestamp: String,
    /// Foreign key reference to the publication.
    pub publication_id: String,
}

impl DataRepoCommits {
    /// Create a new data commit.
    #[must_use]
    pub const fn new(
        commit_hash: String,
        codified_date: Option<String>,
        build_date: Option<String>,
        repo_type: String,
        auth_commit_hash: String,
        auth_commit_timestamp: String,
        publication_id: String,
    ) -> Self {
        Self {
            commit_hash,
            codified_date,
            build_date,
            repo_type,
            auth_commit_hash,
            auth_commit_timestamp,
            publication_id,
        }
    }
}

impl FromRow<'_, AnyRow> for DataRepoCommits {
    fn from_row(row: &AnyRow) -> anyhow::Result<Self, sqlx::Error> {
        Ok(Self {
            commit_hash: row.try_get("commit_hash")?,
            codified_date: row.try_get("codified_date").ok(),
            build_date: row.try_get("build_date").ok(),
            repo_type: row.try_get("repo_type")?,
            auth_commit_hash: row.try_get("auth_commit_hash")?,
            auth_commit_timestamp: row.try_get("auth_commit_timestamp")?,
            publication_id: row.try_get("publication_id")?,
        })
    }
}
