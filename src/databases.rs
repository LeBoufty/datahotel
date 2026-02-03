use crate::userid::{add_entry, dbid_exists, get_user, id_exists};
use rand::prelude::*;
use rusqlite::Connection;
use std::{fs::File, io::Read};

fn get_schema() -> Result<String, std::io::Error> {
    let mut schemafile = File::open("./database.schema")?;
    let mut buf = String::new();
    schemafile.read_to_string(&mut buf)?;
    return Ok(buf);
}

/** Creates a user in the database and returns their ID */
pub fn create_new_user(masterkey: String) -> Result<u32, rusqlite::Error> {
    let mut userid: u32;
    let mut dbid: u32;
    userid = rand::rng().next_u32();
    while id_exists(userid, masterkey.clone())? {
        userid = rand::rng().next_u32();
    }
    dbid = rand::rng().next_u32();
    while dbid_exists(dbid, masterkey.clone())? {
        dbid = rand::rng().next_u32();
    }
    let key = add_entry(userid, dbid, masterkey.clone())?;
    _ = create_database_for_user(dbid, key)?;
    Ok(userid)
}

pub fn get_database(userid: u32, masterkey: String) -> Result<Connection, rusqlite::Error> {
    let user = get_user(userid, masterkey)?;
    let conn = Connection::open(format!("databases/{}.db3", user.dbid()))?;
    conn.execute_batch(
        format!(
            "
        PRAGMA key = '{}';
        PRAGMA cipher_page_size = 4096;
        PRAGMA kdf_iter = 256000;
        PRAGMA cipher_memory_security = ON;
    ",
            user.key()
        )
        .as_str(),
    )?;
    Ok(conn)
}

fn create_database_for_user(dbid: u32, key: String) -> Result<(), rusqlite::Error> {
    let conn = Connection::open(format!("databases/{}.db3", dbid))?;
    let schema = get_schema().expect("Couldn't read schema");
    conn.execute_batch(
        format!(
            "
        PRAGMA key = '{}';
        PRAGMA cipher_page_size = 4096;
        PRAGMA kdf_iter = 256000;
        PRAGMA cipher_memory_security = ON;
    ",
            key
        )
        .as_str(),
    )?;
    _ = conn.execute_batch(schema.as_str());
    Ok(())
}
