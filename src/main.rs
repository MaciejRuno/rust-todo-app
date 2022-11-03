mod args;
mod todo_list;

use std::fs;
use args::{Cli, AddCommand, MarkCommand};
use clap::Parser;
use todo_list::TodoList;


const PATH: &str = "c:/tmp/todo.txt";

fn main() {
    // Get args
    let cli = Cli::parse();

    let mut list: TodoList;
 
    // Read list from fs
    match fs::read_to_string(PATH) {
        Ok(data) => {
            list = serde_json::from_str(&data[..]).unwrap();
        },
        Err(_) => {
            list = TodoList::new(String::from("ToDo List"));
        }
    }
    
    // Do something with the list
    match cli.command {
        args::Command::Add(AddCommand { text, index }) => {
            list.get_index(index).add_item(TodoList::Item{ mark: false, text });
        },
        args::Command::Mark(MarkCommand { index, mark }) => {
            list.get_index(index).mark(mark);
        },
        args::Command::List => {
            list.print();
        },
    }


    // Write list to fs
    let serialized = serde_json::to_string(&list).unwrap();
    fs::write(PATH, serialized).expect("Unable to write file");
}
