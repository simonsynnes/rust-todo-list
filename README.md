# Todo List Built With Rust

This is a Command line tool to create and manage todo list items.
SQLite is used as the database to persist items.

## Commands Available

```
Usage:

    - add [TASK]
        Add's new items
        Example: add "Build a tree"

    - list
        Lists all tasks
        Example: list

    - delete
        Delete all tasks

```

## Development commands:

```sh
# to run once
cargo run <command args>

# to run in watch mode
cargo watch -c -x "run -- <command args>"

# to run tests in watch mode
cargo watch -c -x test
```
