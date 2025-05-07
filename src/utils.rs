use crate::{github::Issue, inputs::InputProvider};
use anyhow::Result;
use colored::Colorize;

pub fn select_issue<'a>(
    issues: &'a [Issue],
    prompt: &str,
    input_provider: &dyn InputProvider,
) -> Result<&'a Issue> {
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
    let selection = input_provider.get_by_select(prompt, &selections)?;

    Ok(&issues[selection])
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_create_short_name_for_a_branch() {
        let issue = Issue {
            number: 48,
            title: "Test issue".to_string(),
            url: "https://github.com/test/tester/issues/48".to_string(),
            labels: vec![],
        };
        let branch_name = create_branch_name_from_issue(&issue);
        assert_eq!(branch_name, "feature/48".to_string());
    }
    #[test]
    fn should_create_pr_text_with_description() {
        let issue_number = 42;
        let description = "This is a test description".to_string();
        let pr = create_pr_text(issue_number, &description);
        assert_eq!(pr, "This is a test description\n\ncloses #42");
    }
    #[test]
    fn test_create_pr_text_without_description() {
        let issue_number = 42;
        let description = "";
        let pr_text = create_pr_text(issue_number, description);
        assert_eq!(pr_text, "closes #42");
    }
}
