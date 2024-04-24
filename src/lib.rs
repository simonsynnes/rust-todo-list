use console::style;
use rusqlite::{Connection, Result};
use std::{
    env, fs,
    path::{Path, PathBuf},
};

pub struct Todo {
    pub id: i32,
    pub text: String,
    pub completed: u8,
    pub date_created: String,
}

// Constructor for new instance of Todo
impl Todo {
    pub fn new(id: i32, text: String, date_created: String, completed: u8) -> Self {
        Todo {
            id,
            text,
            completed,
            date_created,
        }
    }

    pub fn add(conn: &Connection, text: &str) -> Result<()> {
        conn.execute("INSERT INTO todo_list (text) VALUES (?)", &[text])?;
        Ok(())
    }

    pub fn list(conn: &Connection, sort_by_status: bool) -> Result<Vec<Todo>> {
        let sql = if sort_by_status {
            "SELECT * FROM todo_list ORDER BY completed, id"
        } else {
            "SELECT * FROM todo_list ORDER BY id"
        };
        let mut stmt = conn.prepare(sql)?;
        let todo_iter = stmt.query_map((), |row| {
            Ok(Todo::new(
                row.get(0)?,
                row.get(1)?,
                row.get(2)?,
                row.get(3)?,
            ))
        })?;

        let mut todos = Vec::new();
        for todo in todo_iter {
            todos.push(todo?);
        }
        Ok(todos)
    }

    pub fn print_list(todo_list: Vec<Todo>) -> Result<()> {
        for t in todo_list {
            let status = if t.completed == 1 {
                style("Done").green()
            } else {
                style("Pending").yellow()
            };

            println!(
                "{:>4} | {:<44} {:<8} {}",
                style(t.id).cyan().bright(),
                style(truncate_at(&t.text, 12)).bright(),
                status,
                style(t.date_created).dim()
            );
        }
        Ok(())
    }

    pub fn approve_item(conn: &Connection, id: i32) -> Result<()> {
        let mut stmt = conn.prepare("UPDATE todo_list SET completed = 1 WHERE id =?")?;
        stmt.execute(&[&id])?;

        Ok(())
    }

    pub fn reset(conn: &Connection) -> Result<()> {
        let mut stmt = conn.prepare("DELETE FROM todo_list")?;
        stmt.execute([])?;

        Ok(())
    }
}
pub fn truncate_at(input: &str, max_chars: u32) -> String {
    let max_length = max_chars as usize;
    if input.len() > max_length {
        let truncated = &input[..max_length - 3];
        let ellipsis = "...";
        format!("{}{}", truncated, ellipsis)
    } else {
        input.to_string()
    }
}

pub fn help() -> Result<()> {
    let text = r#"Usage
    todo add <text>
    todo list
    todo reset
    todo remove <text>
    todo help
    "#;
    println!("{}", text);

    Ok(())
}

// find home dir for current user
fn get_home_dir() -> String {
    let home = match env::var("HOME") {
        Ok(home_path) => PathBuf::from(home_path),
        Err(_) => {
            // fallback for MacOS and Windows
            if cfg!(target_os = "macos") {
                let home = env::var("HOME").unwrap_or("".to_string());
                PathBuf::from(home)
            } else if cfg!(target_os = "windows") {
                if let Some(userprofile) = env::var("USERPROFILE").ok() {
                    PathBuf::from(userprofile)
                } else if let Some(homedrive) = env::var("HOMEDRIVE").ok() {
                    let homepath = env::var("HOMEPATH").unwrap_or("".to_string());
                    PathBuf::from(format!("{}{}", homedrive, homepath))
                } else {
                    panic!("Couldn't find home directory");
                }
            } else {
                panic!("Could not determine operating system");
            }
        }
    };

    match home.to_str() {
        Some(home_str) => home_str.to_string(),
        None => panic!("Could not convert home dir to str"),
    }
}

// create directory to store db
pub fn verify_db_path(path: &str) -> Result<()> {
    let db_path = Path::new(path);
    if !db_path.exists() {
        // Check if it does not exist
        match fs::create_dir(path) {
            Ok(_) => println!("Successfully created database folder {}", path),

            Err(e) => eprintln!("Error creating database folder {}", e),
        }
    }
    Ok(())
}

// create table if not exists
pub fn verify_db(conn: &Connection) -> Result<()> {
    conn.execute(
        "CREATE TABLE IF NOT EXISTS todo_list (
        id  INTEGER NOT NULL,
        text  TEXT NOT NULL,
        date_created REAL NOT NULL DEFAULT current_timestamp,
        completed NUMERIC NOT NULL DEFAULT 0,
        PRIMARY KEY(id AUTOINCREMENT))",
        [],
    )?;
    Ok(())
}

// get and verify db connection, creating table if not exists
pub fn get_connection() -> Result<Connection> {
    let db_dir = get_home_dir() + "/" + "todo_db/";
    let db_file_path = db_dir.clone() + "todo.sqlite";
    verify_db_path(&db_dir)?;
    let conn = Connection::open(&db_file_path)?;
    verify_db(&conn)?;
    Ok(conn)
}
