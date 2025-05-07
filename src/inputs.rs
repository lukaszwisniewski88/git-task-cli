use anyhow::{Context, Result};
use dialoguer::{Input, Select};

pub trait InputProvider {
    /// Gets a text input from the user with an optional default value
    fn get_input(&self, prompt: &str, default: Option<&str>) -> Result<String>;
    /// Gets a selection from the user from a list of options
    fn get_by_select(&self, prompt: &str, items: &[String]) -> Result<usize>;
}

pub struct ConsoleInput;

impl InputProvider for ConsoleInput {
    fn get_input(&self, prompt: &str, default: Option<&str>) -> Result<String> {
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
    fn get_by_select(&self, prompt: &str, items: &[String]) -> Result<usize> {
        Select::new()
            .with_prompt(prompt)
            .items(items)
            .interact()
            .with_context(|| "Failed to get user input")
    }
}

#[cfg(test)]
pub struct MockInput {
    responses: Vec<String>,
    selections: Vec<usize>,
    resp_index: std::cell::Cell<usize>,
    sel_index: std::cell::Cell<usize>,
}

#[cfg(test)]
impl MockInput {
    pub fn new(responses: Vec<String>, selections: Vec<usize>) -> Self {
        MockInput {
            responses,
            selections,
            resp_index: std::cell::Cell::new(0),
            sel_index: std::cell::Cell::new(0),
        }
    }
}

#[cfg(test)]
impl InputProvider for MockInput {
    fn get_input(&self, _prompt: &str, _default: Option<&str>) -> Result<String> {
        let index = self.resp_index.get();
        if index < self.responses.len() {
            self.resp_index.set(index + 1);
            Ok(self.responses[index].clone())
        } else {
            Err(anyhow::anyhow!("No more mock responses"))
        }
    }
    fn get_by_select(&self, _prompt: &str, _items: &[String]) -> Result<usize> {
        let index = self.sel_index.get();
        if index < self.selections.len() {
            self.sel_index.set(index + 1);
            Ok(self.selections[index])
        } else {
            Err(anyhow::anyhow!("No more mock selections"))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mock_input_provider() {
        // Create mock with predefined responses
        let mock = MockInput::new(
            vec!["First response".to_string(), "Second response".to_string()],
            vec![],
        );

        // Test first call returns first response
        let result1 = mock.get_input("Any prompt", None).unwrap();
        assert_eq!(result1, "First response");

        // Test second call returns second response
        let result2 = mock.get_input("Another prompt", None).unwrap();
        assert_eq!(result2, "Second response");

        // Test that trying to get more responses produces an error
        let result3 = mock.get_input("Third prompt", None);
        assert!(result3.is_err());
    }

    #[test]
    fn test_mock_select_provider() {
        // Create mock with predefined selections
        let mock = MockInput::new(
            vec!["unused".to_string()], // Text responses
            vec![1, 0],                 // Selection indices
        );

        // Test first selection returns 1
        let result1 = mock
            .get_by_select(
                "Select one:",
                &vec!["Option 1".to_string(), "Option 2".to_string()],
            )
            .unwrap();
        assert_eq!(result1, 1);

        // Test second selection returns 0
        let result2 = mock
            .get_by_select(
                "Select another:",
                &vec!["Option A".to_string(), "Option B".to_string()],
            )
            .unwrap();
        assert_eq!(result2, 0);

        // Test that trying to get more selections produces an error
        let result3 =
            mock.get_by_select("Select a third:", &vec!["X".to_string(), "Y".to_string()]);
        assert!(result3.is_err());
    }
}
