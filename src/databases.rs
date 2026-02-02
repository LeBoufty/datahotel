use crate::userid::{add_entry, dbid_exists, get_user_dbid, id_exists};
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
pub fn create_new_user() -> Result<u32, rusqlite::Error> {
    let mut userid: u32;
    let mut dbid: u32;
    userid = rand::rng().next_u32();
    while id_exists(userid)? {
        userid = rand::rng().next_u32();
    }
    dbid = rand::rng().next_u32();
    while dbid_exists(dbid)? {
        dbid = rand::rng().next_u32();
    }
    _ = add_entry(userid, dbid)?;
    _ = create_database_for_user(dbid)?;
    Ok(userid)
}

pub fn get_database(userid: u32) -> Result<Connection, rusqlite::Error> {
    let dbid = get_user_dbid(userid)?;
    let conn = Connection::open(format!("databases/{}.db3", dbid))?;
    Ok(conn)
}

fn create_database_for_user(dbid: u32) -> Result<(), rusqlite::Error> {
    let conn = Connection::open(format!("databases/{}.db3", dbid))?;
    let schema = get_schema().expect("Couldn't read schema");
    _ = conn.execute_batch(schema.as_str());
    _ = conn.close();
    Ok(())
}
