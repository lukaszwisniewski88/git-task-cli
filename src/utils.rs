use anyhow::{Context, Result};
use colored::Colorize;
use dialoguer::{Input, Select};

use crate::github::Issue;

pub fn select_issue<'a>(issues: &'a [Issue], prompt: &str) -> Result<&'a Issue> {
    let selections: Vec<String> = issues
        .iter()
        .map(|issue| {
            let label_str = if !issue.labels.is_empty() {
                let label_names: Vec<String> = issue
                    .labels
                    .iter()
                    .map(|l| format!("[{}]", l.name.green()))
                    .collect();
                format!(" {}", label_names.join(" "))
            } else {
                String::new()
            };
            format!(
                "#{} {} {}",
                issue.number.to_string().blue(),
                issue.title,
                label_str
            )
        })
        .collect();
    if selections.is_empty() {
        return Err(anyhow::anyhow!("No issues available to select"));
    }
    let selection = Select::new()
        .with_prompt(prompt)
        .items(&selections)
        .interact()
        .with_context(|| "Failed to get user input")?;

    Ok(&issues[selection])
}

pub fn get_input(prompt: &str, default: Option<&str>) -> Result<String> {
    if let Some(default_value) = default {
        let input: String = Input::new()
            .with_prompt(prompt)
            .default(default_value.to_string())
            .interact()
            .with_context(|| "Failed to get user input")?;
        Ok(input)
    } else {
        let input: String = Input::new()
            .with_prompt(prompt)
            .interact()
            .with_context(|| "Failed to get user input")?;
        Ok(input)
    }
}

pub fn create_branch_name_from_issue(issue: &Issue) -> String {
    format!("feature/{}", issue.number)
}

pub fn create_pr_text(issue_number: u64, desc: &str) -> String {
    let mut text = String::new();
    if !desc.is_empty() {
        text.push_str(desc);
        text.push_str("\n\n");
    }
    text.push_str(&format!("closes #{}", issue_number));
    text
}
