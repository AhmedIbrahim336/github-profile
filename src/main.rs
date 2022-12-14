mod github;

use dotenv::dotenv;
use github::{GitHub, GitHubErr};
use inquire::{InquireError, Select, Text};
use std::{
    env::{self, VarError},
    fmt::Display,
};
use thiserror::Error;

#[derive(Debug, Error)]
enum AppErr {
    #[error("Missing github auth token")]
    AuthTokenErr(#[from] VarError),
    #[error("Failed to get user input")]
    InputErr(#[from] InquireError),
    #[error("GitHub Error")]
    GitHubErr(#[from] GitHubErr),
}
#[derive(Debug)]
enum Actions {
    Search,
    UserInfo,
}

impl Display for Actions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let txt = match self {
            Actions::Search => "Search users...",
            Actions::UserInfo => "User profile",
        };

        write!(f, "{}", txt)
    }
}

#[tokio::main]
async fn main() -> Result<(), AppErr> {
    dotenv().ok();
    let auth_token = env::var("GITHUB_TOKEN")?;
    let username = env::var("GITHUB_USERNAME")?;
    let github = GitHub::new(&auth_token, &username);

    loop {
        let action = Select::new("GitHub", vec![Actions::Search, Actions::UserInfo]).prompt()?;

        match action {
            Actions::Search => {
                let query = Text::new("User Id").prompt()?;
                println!("Loading...");
                let res = github.search_users(&query).await?;
                if res.total_count == 0 {
                    println!("🤚 No match for {} ", query);
                    continue;
                }
                println!("Total Count: {}", res.total_count);
                let user = Select::new("Search Result", res.items)
                    .with_page_size(10)
                    .prompt()?;
                println!("Loading...");
                match github.user(&user.login).await {
                    Ok(user) => {
                        println!("{}", user);
                    }
                    Err(e) => println!("{:?}", e),
                };
            }
            Actions::UserInfo => {
                let username = Text::new("Username").prompt()?;
                println!("Loading info for {}...", username);
                match github.user(&username).await {
                    Ok(user) => {
                        println!("{}", user);
                    }
                    Err(e) => println!("{:?}", e),
                };
            }
        }
    }
}
