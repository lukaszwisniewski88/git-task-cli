use anyhow::{Context, Result, anyhow};
use octocrab::{Octocrab, params};
use serde::{Deserialize, Serialize};

pub struct GitHubClient {
    client: Octocrab,
    owner: String,
    repo: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Issue {
    pub number: u64,
    pub title: String,
    pub url: String,
    pub labels: Vec<Label>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Label {
    pub name: String,
    pub color: String,
}

impl GitHubClient {
    pub fn new(token: &str, owner: String, repo: String) -> Result<Self> {
        let client = Octocrab::builder()
            .personal_token(token.to_string())
            .build()
            .with_context(|| "Failed to create GithubClient")?;

        Ok(GitHubClient {
            client,
            owner,
            repo,
        })
    }
    pub async fn list_open_issues(&self) -> Result<Vec<Issue>> {
        let issues = self
            .client
            .issues(&self.owner, &self.repo)
            .list()
            .state(params::State::Open)
            .send()
            .await
            .with_context(|| "Failed to fetch open issues")?;
        let mut result = Vec::new();

        for issue in issues {
            result.push(Issue {
                number: issue.number,
                title: issue.title,
                url: issue.html_url.to_string(),
                labels: issue
                    .labels
                    .into_iter()
                    .map(|l| Label {
                        name: l.name,
                        color: l.color,
                    })
                    .collect(),
            });
        }
        Ok(result)
    }
    pub async fn add_label_to_issue(&self, issue_number: u64, label: &str) -> Result<()> {
        self.client
            .issues(&self.owner, &self.repo)
            .add_labels(issue_number, &[label.to_string()])
            .await
            .with_context(
                || format!("Failed to add label {} to issue #{}", label, issue_number,),
            )?;
        Ok(())
    }
    pub async fn remove_label_from_issue(&self, issue_number: u64, label: &str) -> Result<()> {
        self.client
            .issues(&self.owner, &self.repo)
            .remove_label(issue_number, label.to_string())
            .await
            .with_context(|| {
                format!(
                    "Failed to remove lael {} from issue #{}",
                    label, issue_number
                )
            })?;
        Ok(())
    }
    pub async fn create_pull_request(
        &self,
        title: &str,
        body: &str,
        head: &str,
        base: &str,
    ) -> Result<String> {
        let pr = self
            .client
            .pulls(&self.owner, &self.repo)
            .create(title, head, base)
            .body(body)
            .send()
            .await
            .with_context(|| "Failed to create a pull request")?;
        match pr.html_url {
            Some(url) => Ok(url.to_string()),
            None => Err(anyhow!("Failed to get pull request URL")),
        }
    }
}
