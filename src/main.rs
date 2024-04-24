use std::env;

use dialoguer::Confirm;
use rusqlite::Result;
use rust_todo_list::{get_connection, help, Todo};

fn main() -> Result<()> {
    println!("Rust todo list");

    let args: Vec<String> = env::args().collect();

    let connection = get_connection()?;

    if args.len() == 1 {
        println!("You need to specify any arguments!");
        help()?;
        std::process::exit(1);
    }

    let command = &args[1];
    let suffix = &args[2..].iter().cloned().collect::<Vec<_>>().join(" ");

    match command.as_str() {
        "add" => {
            if !suffix.as_str().is_empty() {
                Todo::add(&connection, suffix.as_str())?;
            } else {
                help()?;
                std::process::exit(1);
            }
            Ok(())
        }
        "list" => {
            let todo_list = Todo::list(&connection, true)?;
            Todo::print_list(todo_list)?;
            Ok(())
        }
        "approve" => {
            if !suffix.as_str().is_empty() {
                let id = args[2].parse::<i32>().unwrap();
                Todo::approve_item(&connection, id)?;
            } else {
                help()?;
                std::process::exit(1);
            }
            Ok(())
        }
        "delete" => {
            if !suffix.as_str().is_empty() {
                let id = args[2].parse::<i32>().unwrap();
                Todo::delete(&connection, id)?;
            } else {
                help()?;
                std::process::exit(1);
            }
            Ok(())
        }
        "reset" => {
            let confirmation = Confirm::new()
                .with_prompt("Are you sure you want to reset the list?")
                .interact()
                .unwrap();

            if confirmation {
                Todo::reset(&connection)?;
            } else {
                println!("Not deleting");
            }
            Ok(())
        }
        "help" | "--help" | "-h" | _ => help(),
    }?;

    Ok(())
}
