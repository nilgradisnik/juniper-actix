use rusqlite::{Connection, Result, NO_PARAMS};

use crate::schema::user::{User, NewUser};

#[derive(Copy, Clone)]
pub struct Sqlite {
    pub db_path: &'static str
}

impl juniper::Context for Sqlite {}

impl Sqlite {
    pub fn get_user(&self, id: i32) -> Result<User> {
        let conn = Connection::open(self.db_path)?;

        let user = select_user(&conn, id)?;

        Ok(user)
    }

    pub fn add_user(&self, user: NewUser) -> Result<User> {
        let conn = Connection::open(self.db_path)?;

        let user_id = insert_user(&conn, user)?;

        let user = select_user(&conn, user_id)?;

        Ok(user)
    }
}

fn select_user(conn: &Connection, user_id: i32) -> Result<User> {
    conn.query_row_named(
        "SELECT id, first_name, last_name, email FROM users WHERE id = :id",
        &[
            (":id", &user_id)
        ],
        |row| Ok(User {
            id: row.get(0)?,
            first_name: row.get(1)?,
            last_name: row.get(2)?,
            email: row.get(3)?,
        })
    )
}

fn insert_user(conn: &Connection, user: NewUser) -> Result<i32> {
    conn.execute_named(
        "INSERT INTO users (first_name, last_name, email) VALUES (:first_name, :last_name, :email)",
        &[
            (":first_name", &user.first_name),
            (":last_name", &user.last_name),
            (":email", &user.email)
        ],
    )?;

    Ok(conn.last_insert_rowid() as i32)
}

fn _create_user_table(conn: &Connection) -> Result<usize> {
    conn.execute(
        "CREATE TABLE users (
            id              INTEGER PRIMARY KEY,
            first_name      TEXT NOT NULL,
            last_name       TEXT NOT NULL,
            email           TEXT NOT NULL UNIQUE
        )",
        NO_PARAMS,
    )
}