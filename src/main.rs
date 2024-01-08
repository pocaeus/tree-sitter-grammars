use std::path::PathBuf;

use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,

    /// Path to file containing languages and their grammar repositories
    #[arg(short, long, default_value = "./languages.toml", global = true)]
    file: PathBuf,

    /// Path to directory containing grammar repositories
    #[arg(short, long, default_value = "./grammars/", global = true)]
    directory: PathBuf,
}

#[derive(Subcommand)]
enum Commands {
    /// Add a new tree-sitter grammar to the `languages.toml` file
    Add {
        /// Name of the language being added, e.g. 'rust'
        #[arg(short, long)]
        name: String,
        /// URL to the tree-sitter grammar, e.g. 'git@github.com:tree-sitter/tree-sitter-rust.git'
        #[arg(short, long)]
        git: String,
        /// Optional git hash to checkout from the grammar repository
        #[arg(long)]
        hash: Option<String>,
    },
    /// Update the tree-sitter grammar(s)
    Update {
        /// Name of the language grammar to update, e.g. 'rust'
        #[arg(short, long)]
        name: Option<String>,
        /// Use this flag to update all grammars for all languages listed
        #[arg(long, default_value_t = false)]
        all: bool,
    },
}

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Some(Commands::Add { name, git, hash }) => {
            println!("Add");
        }
        Some(Commands::Update { name, all }) => {
            println!("Update")
        }
        None => {}
    }
}
