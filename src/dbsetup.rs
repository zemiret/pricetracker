mod storage;
mod config;

use config::{SKALNIK_TEST_URL, A8A_TEST_URL};
use rusqlite::Connection;

fn main() {
    let conn = Connection::open("db.db3").expect("could not open DB"); 
    storage::create_tables(&conn).expect("Error creating tables");

    // TODO: Just for testing
    storage::add_watch_item(&conn, SKALNIK_TEST_URL).unwrap();
    storage::add_watch_item(&conn, A8A_TEST_URL).unwrap();
}