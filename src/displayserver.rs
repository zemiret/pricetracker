mod config;
mod storage;

use std::sync::Mutex;

use rusqlite::Connection;
use rouille::router;
use rouille::Response;

fn main() {
    let conn = {
        let conn = Connection::open("db.db3");
        Mutex::new(conn.expect("could not open DB"))
    };


    rouille::start_server("0.0.0.0:8080", move |request| {
        router!(request,
            (GET) (/api/v1/entries) => {

                // TODO: DON'T DO THUS WITH UNWRAP and expect! (as they may panic)
                let page_info_map = storage::get_entries(&conn.lock().unwrap()).expect("storage.get_entries");

                // TODO: Render to JSON
                Response::text(format!("{:#?}", page_info_map))
            },
            _ => rouille::Response::empty_404()
        )
    });
}
