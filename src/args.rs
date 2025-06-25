use clap::{Args, Parser, Subcommand};

#[derive(Parser, Debug)]
#[clap(author, version, about)]
pub struct Cli {
    #[clap(subcommand)]
    pub command: Command,
}

#[derive(Subcommand, Debug)]
pub enum Command {
    /// Add a list element
    Add(AddCommand),
    /// Mark or unmark a list element
    Mark(MarkCommand),
    /// Show list
    List,
}

#[derive(Args, Debug)]
pub struct AddCommand {
    /// Element label
    pub text: String,
    #[arg(short, long, default_value_t = 0)]
    /// Index to put element under
    pub index: usize,
}

#[derive(Args, Debug)]
pub struct MarkCommand {
    /// Index to modify
    pub index: usize,
    #[arg(short, long, default_value_t = true)]
    /// Mark or unmark
    pub mark: bool,
}
