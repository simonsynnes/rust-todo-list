use std::env;

use rust_todo_list::{get_connection, help};
use rusqlite::Result;


pub struct TodoList {
    pub id: u32,
    pub text: String,
    pub completed: bool,
    pub date_created: String,
}

fn add(suffix: &str) {
    println!("adding")
}

fn remove(suffix: &str) {
}

fn list() {
    let args: Vec<String> = env::args().collect();
    println!("{:?}", args);
}
fn main() -> Result<()> {
    println!("Rust todo list");

    let args:Vec<String> = env::args().collect();

    let connection = get_connection()?;

    if args.len() == 0 {
        println!("You need to specify any arguments!");
        help();
        std::process::exit(1);
    }

    let command = &args[1];
    let suffix = &args[2..].iter().cloned().collect::<Vec<_>>().join(" ");

    match command.as_str() {
        "add" => add(suffix),
        "list" => list(),
        "remove" => remove(suffix),
        "help" => help(),
        _ => println!("Unknown command: {}", command),
    }

    Ok(())

}
