// github 
mod auth;
pub use auth::{main as authenticate_github};

// config
mod config;
pub use config::{AuthorConfig, Config, GitHubConfig, ModelConfig};

// watch 
mod watch;