use rand::distr::{Alphanumeric, SampleString};
use rusqlite::Connection;

pub struct User {
    id: u32,
    dbid: u32,
    key: String,
}

impl User {
    pub fn id(&self) -> u32 {
        self.id
    }
    pub fn dbid(&self) -> u32 {
        self.dbid
    }
    pub fn key(&self) -> String {
        self.key.clone()
    }
}

fn get_userid_db(masterkey: String) -> Result<Connection, rusqlite::Error> {
    let conn = Connection::open("./index.db3")?;
    conn.execute_batch(
        format!(
            "
        PRAGMA key = '{}';
        PRAGMA cipher_page_size = 4096;
        PRAGMA kdf_iter = 256000;
        PRAGMA cipher_memory_security = ON;
    ",
            masterkey
        )
        .as_str(),
    )?;
    if !conn.table_exists(None, "users")? {
        conn.execute(
            "CREATE TABLE users(id INTEGER UNIQUE, dbid INTEGER UNIQUE, KEY TEXT);",
            [],
        )?;
    }
    return Ok(conn);
}

pub fn get_usercount(masterkey: String) -> Result<u32, rusqlite::Error> {
    let conn = get_userid_db(masterkey)?;
    let count = conn.query_row("SELECT COUNT(id) FROM users", [], |row| row.get(0))?;
    return Ok(count);
}

pub fn get_user(id: u32, masterkey: String) -> Result<User, rusqlite::Error> {
    let conn = get_userid_db(masterkey)?;
    let user =
        conn.query_row_and_then("SELECT (dbid, key) FROM users WHERE id = ?1", [id], |row| {
            Ok(User {
                id,
                dbid: row.get(0)?,
                key: row.get(1)?,
            })
        });
    return user;
}

pub fn dbid_exists(dbid: u32, masterkey: String) -> Result<bool, rusqlite::Error> {
    let conn = get_userid_db(masterkey)?;
    let count: u32 = conn.query_row(
        "SELECT COUNT(id) FROM users WHERE dbid = ?1",
        [dbid],
        |row| row.get(0),
    )?;
    return Ok(count != 0);
}

pub fn id_exists(id: u32, masterkey: String) -> Result<bool, rusqlite::Error> {
    let conn = get_userid_db(masterkey)?;
    let count: u32 = conn.query_row("SELECT COUNT(id) FROM users WHERE id = ?1", [id], |row| {
        row.get(0)
    })?;
    return Ok(count != 0);
}

/** Adds an entry to the user registry then returns the key */
pub fn add_entry(id: u32, dbid: u32, masterkey: String) -> Result<String, rusqlite::Error> {
    let conn = get_userid_db(masterkey)?;
    let key = Alphanumeric.sample_string(&mut rand::rng(), 16);
    _ = conn.execute(
        format!(
            "INSERT INTO users (id, dbid, key) VALUES (?1, ?2, '{}')",
            key
        )
        .as_str(),
        [id, dbid],
    );
    return Ok(key);
}
