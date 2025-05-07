use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(name = "git-issue-flow")]
#[command(about = "A CLI tool for managing Git issues", long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    Config {
        /// set the GITHUB token
        #[arg(long)]
        token: Option<String>,
    },
    /// Start working on the issue
    Start,
    Finish {
        title: Option<String>,

        #[arg(short, long)]
        description: Option<String>,
    },
    List,
}

pub fn parse_args() -> Cli {
    Cli::parse()
}
