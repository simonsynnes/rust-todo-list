use std::env;

use rust_todo_list::help;


fn add(suffix: &str) {
    println!("adding")
}

fn remove(suffix: &str) {
}

fn list() {
    let args: Vec<String> = env::args().collect();
    println!("{:?}", args);
}
fn main() {
    println!("Rust todo list");

    let args:Vec<String> = env::args().collect();

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

}
