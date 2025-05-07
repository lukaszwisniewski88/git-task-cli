use anyhow::{Context, Result, anyhow};
use git2::Repository;

pub struct GitRepo {
    repo: Repository,
}

impl GitRepo {
    pub fn open() -> Result<Self> {
        let repo = Repository::open_from_env()
            .with_context(|| "Failed to open git repository! Are you in a git repo?")?;
        Ok(GitRepo { repo })
    }
    pub fn get_github_remote_url(&self) -> Result<String> {
        let remote = self
            .repo
            .find_remote("origin")
            .with_context(|| "Failed to find origin remote")?;
        let url = remote
            .url()
            .ok_or_else(|| anyhow!("Remote URL is not valid UTF-8"))?;
        let url = normalize_github_url(url);
        Ok(url)
    }
    pub fn get_current_branch_name(&self) -> Result<String> {
        let head = self
            .repo
            .head()
            .with_context(|| "Failed to get HEAD reference")?;
        let name = head
            .name()
            .ok_or_else(|| anyhow!("Failed to get branch name"))?;
        Ok(name.to_string())
    }
    pub fn get_repo_owner_and_name(&self) -> Result<(String, String)> {
        let url = self.get_github_remote_url()?;
        parse_github_owner_and_repo(&url)
    }
    pub fn create_branch(&self, name: &str) -> Result<()> {
        let head = self
            .repo
            .head()
            .with_context(|| "Failed to get HEAD reference")?;
        let commit = head
            .peel_to_commit()
            .with_context(|| "Failed to get HEAD commit")?;
        self.repo
            .branch(name, &commit, false)
            .with_context(|| format!("Failed to create branch: {name}"))?;
        let obj = self
            .repo
            .revparse_single(&format!("refs/heads/{name}"))
            .with_context(|| format!("Failed to get reference to the new branch: {name}"))?;

        self.repo
            .checkout_tree(&obj, None)
            .with_context(|| format!("Failed to checkout tree for branch: {name}"))?;
        self.repo
            .set_head(&format!("refs/heads/{name}"))
            .with_context(|| format!("Failed to set HEAD to new branch: {name}"))?;
        Ok(())
    }
}

fn normalize_github_url(url: &str) -> String {
    if url.starts_with("git@github.com:") {
        // Convert SSH URL to HTTPS URL
        return url
            .replacen("git@github.com:", "https://github.com/", 1)
            .trim_end_matches(".git")
            .to_string();
    }

    // Already HTTPS or other format, just trim .git
    url.trim_end_matches(".git").to_string()
}

fn parse_github_owner_and_repo(url: &str) -> Result<(String, String)> {
    // Handle HTTPS URLs like https://github.com/owner/repo
    println!("Parsing the url from your repo: {url}");
    if url.starts_with("https://github.com/") {
        let path = url.trim_start_matches("https://github.com/");
        let parts: Vec<&str> = path.split('/').collect();

        if parts.len() >= 2 {
            return Ok((parts[0].to_string(), parts[1].to_string()));
        }
    }

    // Handle SSH URLs like git@github.com:owner/repo.git
    if url.starts_with("git@github.com:") {
        let path = url.trim_start_matches("git@github.com:");
        let path = path.trim_end_matches(".git");
        let parts: Vec<&str> = path.split('/').collect();

        if parts.len() >= 2 {
            return Ok((parts[0].to_string(), parts[1].to_string()));
        }
    }

    Err(anyhow!(
        "Could not parse GitHub owner and repo from URL: {}",
        url
    ))
}
