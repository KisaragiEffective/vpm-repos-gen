use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error(transparent)]
    Octocrab(#[from] octocrab::Error),

    #[error(transparent)]
    Reqwest(#[from] reqwest::Error),

    #[error(transparent)]
    SerdeJson(#[from] serde_json::Error),

    #[error("Invalid package.json")]
    InvalidPackageJson,

    #[error("Invalid repo")]
    InvalidRepo,
}
