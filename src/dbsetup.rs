mod storage;

use rusqlite::Connection;

fn main() {
    let conn = Connection::open("db.db3").expect("could not open DB"); 
    storage::create_tables(&conn).expect("Error creating tables");
}