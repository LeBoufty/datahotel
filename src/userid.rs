use rusqlite::Connection;

fn get_userid_db() -> Result<Connection, rusqlite::Error> {
    let conn = Connection::open("./index.db3")?;
    if !conn.table_exists(None, "users")? {
        conn.execute(
            "CREATE TABLE users(id INTEGER UNIQUE, dbid INTEGER UNIQUE);",
            [],
        )?;
    }
    return Ok(conn);
}

pub fn get_usercount() -> Result<u32, rusqlite::Error> {
    let conn = get_userid_db()?;
    let count = conn.query_row("SELECT COUNT(id) FROM users", [], |row| row.get(0))?;
    return Ok(count);
}

pub fn get_user_dbid(id: u32) -> Result<u32, rusqlite::Error> {
    let conn = get_userid_db()?;
    let dbid: u32 =
        conn.query_row_and_then("SELECT dbid FROM users WHERE id = ?1", [id], |row| {
            row.get(0)
        })?;
    return Ok(dbid);
}

pub fn dbid_exists(dbid: u32) -> Result<bool, rusqlite::Error> {
    let conn = get_userid_db()?;
    let count: u32 = conn.query_row(
        "SELECT COUNT(id) FROM users WHERE dbid = ?1",
        [dbid],
        |row| row.get(0),
    )?;
    return Ok(count != 0);
}

pub fn id_exists(id: u32) -> Result<bool, rusqlite::Error> {
    let conn = get_userid_db()?;
    let count: u32 = conn.query_row("SELECT COUNT(id) FROM users WHERE id = ?1", [id], |row| {
        row.get(0)
    })?;
    return Ok(count != 0);
}

pub fn add_entry(id: u32, dbid: u32) -> Result<usize, rusqlite::Error> {
    let conn = get_userid_db()?;
    return conn.execute("INSERT INTO users (id, dbid) VALUES (?1, ?2)", [id, dbid]);
}
