# Git Task CLI

Git Task CLI is a powerful command-line tool that streamlines your GitHub issue workflow in Git repositories. It helps developers manage issues, create topic branches, and automate pull request creation - all from the command line.

## Problem Statement

Working with GitHub issues often involves several manual steps:
1. Selecting an issue to work on
2. Creating a branch with a consistent naming convention
3. Marking the issue as being worked on (usually with a label)
4. Creating a pull request when finished
5. Linking the PR to the issue
6. Updating issue status/labels

This CLI tool automates this entire workflow, saving time and ensuring consistency.

## Features

- 🔍 **List Issues**: View all open issues in your GitHub repository
- 🚀 **Start Issues**: Select an issue, automatically create a branch, and mark it as in-progress
- 🎯 **Finish Issues**: Create pull requests linked to the original issue with minimal effort
- 🔐 **Token Management**: Secure GitHub token configuration

## Installation

### Prerequisites

- Rust and Cargo installed on your system
- Git installed and configured
- A GitHub account and personal access token

### Building from Source

```bash
# Clone the repository
git clone https://github.com/username/git-task-cli.git

# Navigate to the project directory
cd git-task-cli

# Build the project
cargo build --release

# Optional: Move the binary to your PATH
cp target/release/git-issue-flow /usr/local/bin/

```

## Usage
### Configuration

Before using Git Task CLI, you need to configure your GITHUB token, it should have access to PRs and issues of the repository you want to work on
```bash
git-issue-flow config --token <YOUR_GITHUB_TOKEN>

```
to view the current token configuration:
```bash

git-issue-flow config
```
### Listing Issues

To list all open issues in the current repository:

```bash
git-issue-flow list
```

This will display issues with their numbers, titles, and labels.

### Starting Work on an Issue

To start working on an issue:

```bash
git-issue-flow start
```

This will:
1. Present a list of open issues to choose from
2. Create a new branch named `feature/<issue-number>`
3. Add the `working-on` label to the selected issue
4. Switch to the newly created branch

### Finishing Work on an Issue

When you're ready to create a pull request:

```bash
git-issue-flow finish
```

You can also specify a title and description for the PR:

```bash
git-issue-flow finish --title "Your PR title" --description "Detailed description of changes"
```

This will:
1. Create a pull request from your current branch to the main branch
2. Link the PR to the issue with a "closes #<issue-number>" reference
3. Remove the `working-on` label from the issue

## How It Works

Git Task CLI integrates with:
- Local Git repositories via the `git2` crate
- GitHub API via the `octocrab` crate
- Terminal UI elements via `dialoguer` and `colored` crates

The tool automatically:
- Extracts repository owner and name from your git remotes
- Authenticates with GitHub using your provided token
- Creates standardized branch names and PR descriptions
- Manages issue labels to reflect work status

## Example Workflow

```bash
# 1. List available issues
git-issue-flow list

# 2. Start working on an issue (interactive selection)
git-issue-flow start

# 3. Make your changes and commit them
git add .
git commit -m "Implement feature X"

# 4. Create a PR to finish the task
git-issue-flow finish
```

## License

[MIT License][https://opensource.org/license/mit]

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.
