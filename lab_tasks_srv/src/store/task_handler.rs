use lab_tasks_types::task::{Task, TaskDTO};
use rusqlite::Connection;

use super::sql_mapper::FromSqliteRow;

pub struct TaskHandler {}

impl FromSqliteRow for Task {
    fn from_row(row: &rusqlite::Row) -> Result<Task, rusqlite::Error> {
        Ok(Task::new(
            row.get(0)?,
            row.get(1)?,
            row.get(2)?,
            row.get(3)?,
            row.get(4)?,
            Some(row.get(5)?),
            Some(row.get(6)?),
            Some(row.get(7)?),
            Some(row.get(8)?),
            row.get(9)?,
            row.get(10)?,
            row.get(11)?
        ))
    }
}

impl TaskHandler {
    pub fn new() -> Self {
        Self {}
    }
    pub fn create_store(&self) -> Result<(), rusqlite::Error> {
        let conn = Connection::open("tasks.db")?;
        conn.execute(
            "CREATE TABLE task
                    ( 
                        id INTEGER PRIMARY KEY AUTOINCREMENT,
                        name TEXT NOT NULL,
                        description TEXT NOT NULL,
                        category TEXT NOT NULL,
                        status TEXT NOT NULL,
                        created_at TEXT NOT NULL,
                        updated_at TEXT NOT NULL,
                        completed INTEGER NOT NULL,
                        archived INTEGER NOT NULL,      
                        user INTEGER NOT NULL,
                        deadline TEXT NOT NULL,
                        tel TEXT NOT NULL
                    );
                ",
            (),
        )?;
        Ok(())
    }
    pub fn insert_task(&self, task: &Task) -> Result<(), rusqlite::Error> {
        let conn = Connection::open("tasks.db").unwrap();
        match conn.execute(
            "INSERT INTO task ( name, description, category, status, created_at, updated_at, completed, archived, user, deadline, tel)
                    VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11);",
            (
                task.name(),
                task.description(),
                task.category(),
                task.status(),
                task.created_at(),
                task.updated_at(),
                0,
                0,
                task.user(),
                task.deadline(),
                task.tel()
            ) 
        ){
            Ok(_) => return Ok(()),
            Err(e) => {println!("{}", e); }
        }

        Ok(())
    }
    pub fn delete_task(&self, id: u32) -> Result<(), rusqlite::Error> {
        let conn = Connection::open("tasks.db").unwrap();
        conn.execute("UPDATE task SET archived = 1 WHERE id = ?1", [id])?;
        Ok(())
    }

    pub fn update_task(&self, updated_task: &TaskDTO) -> Result<(), rusqlite::Error> {
        let conn = Connection::open("tasks.db").unwrap();
        conn.execute("UPDATE task SET name = ?1, description = ?2, status = ?3, updated_at = ?4, completed = ?5 WHERE id = ?6",
            (updated_task.name(), updated_task.description(), updated_task.status(), updated_task.updated_at(), updated_task.completed(), updated_task.id()))?;
        Ok(())
    }

    pub fn get_tasks(&self) -> Result<Vec<Task>, rusqlite::Error> {
        Connection::open("tasks.db")?
            .prepare("SELECT * FROM task WHERE archived = 0;")?
            .query_map([], Task::from_row)?
            .collect()
    }

    pub fn get_open_tasks(&self) -> Result<Vec<Task>, rusqlite::Error> {
        Connection::open("tasks.db")?
            .prepare("SELECT * FROM task where completed = 0 AND archived = 0;")?
            .query_map([], Task::from_row)?
            .collect()
    }
}
