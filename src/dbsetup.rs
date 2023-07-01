mod storage;

use rusqlite::Connection;

fn main() {
    let conn = Connection::open("db.db3").expect("could not open DB"); 
    storage::create_tables(&conn).expect("Error creating tables");

    // TODO: Just for testing
    storage::add_watch_item(&conn, "https://www.skalnik.pl/buty-mantra-arctic-flame-1016931").unwrap();
    storage::add_watch_item(&conn, "https://8a.pl/lina-dynamiczna-beal-joker-9-1-mm-60m-unicore-dry-cover-orange").unwrap();
}