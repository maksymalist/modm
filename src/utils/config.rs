use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub model: ModelConfig,
    pub github: GitHubConfig,
    pub author: AuthorConfig,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ModelConfig {
    pub name: String,
    pub version: String,
    pub directory: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GitHubConfig {
    pub repo: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AuthorConfig {
    pub author: String,
    pub email: String,
}
