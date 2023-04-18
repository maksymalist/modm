use std::process::Command;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
struct GitHubUser {
    login: String,
    id: i64,
    node_id: String,
    avatar_url: String,
    gravatar_id: String,
    url: String,
    html_url: String,
    followers_url: String,
    following_url: String,
    gists_url: String,
    starred_url: String,
    subscriptions_url: String,
    organizations_url: String,
    repos_url: String,
    events_url: String,
    received_events_url: String,
    site_admin: bool,
    name: String,
    company: String,
    blog: String,
    location: String,
    email: Option<String>,
    hireable: Option<bool>,
    bio: String,
    twitter_username: String,
    public_repos: i64,
    public_gists: i64,
    followers: i64,
    following: i64,
    created_at: String,
    updated_at: String,
}

pub fn main() {
    let output = Command::new("gh")
        .arg("api")
        .arg("user")
        .output()
        .expect("failed to execute gh command");

    let stdout = String::from_utf8_lossy(&output.stdout);

    let lines: Vec<&str> = stdout.lines().collect();
    let deserialised: GitHubUser = serde_json::from_str(lines[0]).unwrap();
    let user = deserialised.login;

    println!("json: {:?}", user);
}