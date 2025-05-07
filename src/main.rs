mod cli;
mod config;
mod git;
mod github;
mod inputs;
mod utils;

use anyhow::Result;
use cli::{Commands, parse_args};
use colored::Colorize;
use config::Config;
use git::GitRepo;
use github::GitHubClient;
use inputs::{ConsoleInput, InputProvider};
use utils::{create_branch_name_from_issue, create_pr_text, select_issue};
const WORKING_LABEL: &str = "working-on";
#[tokio::main]
async fn main() -> Result<()> {
    let args = parse_args();
    match args.command {
        Commands::Config { token } => {
            println!("Configuring with token: {token:?}");
            let mut config = Config::load()?;
            if let Some(token) = token {
                config.set_github_token(token)?;
                println!("Github token saved successfully!");
            } else if let Some(token) = &config.github_token {
                println!("Github token already set: {}****", &token[0..4]);
            } else {
                println!("Github token not set");
            }
            Ok(())
        }
        Commands::List => {
            println!("Listing tasks");
            list_command().await?;
            Ok(())
        }
        Commands::Start => {
            let input_provider = ConsoleInput;
            start_command(&input_provider).await?;
            Ok(())
        }
        Commands::Finish { title, description } => {
            let input_provider = ConsoleInput;
            finish_command(&input_provider, title, description).await?;
            Ok(())
        }
    }
}

async fn start_command(input_provider: &dyn InputProvider) -> Result<()> {
    let config = Config::load()?;
    let token = config.github_token.ok_or_else(|| {
        anyhow::anyhow!("Github token not found. Please set it with config --token <TOKEN>")
    })?;
    let repo = GitRepo::open()?;
    let (owner, repo_name) = repo.get_repo_owner_and_name()?;

    println!("Fetching issues from {owner} - {repo_name} ! ");
    let client = GitHubClient::new(&token, owner, repo_name)?;
    let issues = client.list_open_issues().await?;
    let selected = select_issue(&issues, "Select an issue to work on", input_provider)?;
    println!("Starting task:#{} {}", selected.number, selected.title);
    client
        .add_label_to_issue(selected.number, WORKING_LABEL)
        .await?;
    let branch_name = create_branch_name_from_issue(selected);
    repo.create_branch(&branch_name)?;
    println!("Created and switched to branch {branch_name}");

    println!("\nYou're all set! Make your changes and when you're ready to create a PR, run:");
    println!("  git-issue-flow finish");
    Ok(())
}

async fn finish_command(
    input_provider: &dyn InputProvider,
    title: Option<String>,
    desc: Option<String>,
) -> Result<()> {
    let config = Config::load()?;
    let token = config.github_token.ok_or_else(|| {
        anyhow::anyhow!("Github token not found!. Please set it up with 'config --token <TOKEN>'")
    })?;
    let repo = GitRepo::open()?;
    let (owner, repo_name) = repo.get_repo_owner_and_name()?;
    let current_branch = repo.get_current_branch_name()?;
    let issue_number = current_branch
        .strip_prefix("feature/")
        .and_then(|num| num.parse::<u64>().ok())
        .ok_or_else(|| {
            anyhow::anyhow!(
                "Current branch {} is not a valid issue branch",
                current_branch
            )
        })?;
    let client = GitHubClient::new(&token, owner, repo_name)?;
    let title = match title {
        Some(t) => t,
        None => input_provider.get_input("Wprowadź tytuł dla PR", None)?,
    };
    let description = match desc {
        Some(d) => d,
        None => input_provider.get_input("Wprowadź opis dla PR", None)?,
    };
    let pr_body = create_pr_text(issue_number, &description);
    let pr_url = client
        .create_pull_request(&title, &pr_body, &current_branch, "main")
        .await?;
    println!("Pull request created: {}", pr_url.blue());

    client
        .remove_label_from_issue(issue_number, WORKING_LABEL)
        .await?;

    Ok(())
}

async fn list_command() -> Result<()> {
    let config = Config::load()?;
    let token = config.github_token.ok_or_else(|| {
        anyhow::anyhow!("Github token not found!. Please set it up with 'config --token <TOKEN>'")
    })?;
    let repo = GitRepo::open()?;
    let (owner, repo_name) = repo.get_repo_owner_and_name()?;
    println!("Fetching issues from {owner} - {repo_name} ");
    let client = GitHubClient::new(&token, owner.clone(), repo_name.clone())?;
    let issues = client.list_open_issues().await?;
    if issues.is_empty() {
        print!("No open issues found");
        return Ok(());
    }
    println!("Open issues in {}/{} - #{}", owner, repo_name, issues.len());
    for issue in issues {
        let labels = if !issue.labels.is_empty() {
            let label_str: Vec<String> = issue
                .labels
                .iter()
                .map(|l| format!("[{}]", l.name))
                .collect();
            format!(" {}", label_str.join(" "))
        } else {
            String::new()
        };
        println!(
            "#{} {}{}",
            issue.number.to_string().red(),
            issue.title.blue(),
            labels.color("#DDDFFA")
        );
    }

    Ok(())
}
