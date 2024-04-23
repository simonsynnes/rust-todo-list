use std::{env, fs, path::{Path, PathBuf}};
use rusqlite::{Connection, Result};

pub fn help() {
    let text = r#"Usage
    todo add <text>
    todo list
    todo remove <text>
    todo help
    "#;
    println!("{}", text);
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
    None =>  panic!("Could not convert home dir to str"),
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
    conn.execute("CREATE TABLE IF NOT EXISTS todo_list {
        id  INTEGER NOT NULL,
        name  TEXT NOT NULL,
        date_added REAL NOT NULL DEFAULT current_timestamp,
        completed NUMERIC NOT NULL DEFAULT 0,
        PRIMARY KEY(id AUTOINCREMENT)", [], )?;
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