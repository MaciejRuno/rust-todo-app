mod args;
mod config;
mod todo_list;

use args::{AddCommand, Cli, MarkCommand};
use clap::Parser;
use config::Config;
use std::fs;
use std::path::PathBuf;
use todo_list::TodoList;

fn todo_path() -> PathBuf {
    let mut path = std::env::temp_dir();
    path.push("todo.txt");
    path
}

fn main() {
    // Load configuration from environment
    let _config = Config::from_env();

    // Get args
    let cli = Cli::parse();

    let mut list: TodoList;

    // Read list from fs
    let path = todo_path();
    match fs::read_to_string(&path) {
        Ok(data) => {
            list = serde_json::from_str(&data[..]).unwrap();
        }
        Err(_) => {
            list = TodoList::new(String::from("ToDo List"));
        }
    }

    // Do something with the list
    match cli.command {
        args::Command::Add(AddCommand { text, index }) => {
            list.get_index(index)
                .add_item(TodoList::Item { mark: false, text });
        }
        args::Command::Mark(MarkCommand { index, mark }) => {
            list.get_index(index).mark(mark);
        }
        args::Command::List => {
            list.print();
        }
    }

    // Write list to fs
    let serialized = serde_json::to_string(&list).unwrap();
    fs::write(path, serialized).expect("Unable to write file");
}
