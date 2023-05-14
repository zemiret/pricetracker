use std::collections::HashMap;

use rusqlite::{Connection,Result, params};

// struct StorageWatchItem {
//     id: i32,
//     url: String,
// }

// struct StorageEntry {
//     price: f64,
//     created_at: i64,
//     item_id: i32,
// }

// Exposing public structs without storage implementations
pub struct Entry {
    price: f64,
    created_at: i64
}

// Exposing public structs without storage implementations
pub struct WatchItem {
    url: String,
}

pub type EntryMap = HashMap<String, Vec<Entry>>;

pub fn create_tables(conn: &Connection) -> Result<()> {
    conn.execute(
        "create table watchitems(
        id integer primary key autoincrement,
        url text
        )",
        [],
    )?;

    conn.execute(
        "create table entries(
        price real,
        created_at integer,
        item_id integer,
        foreign key (item_id) references watchitems(id)
    )",
        [],
    )?;

    Ok(())
}

fn add_watch_item(conn: &Connection, url: &str) -> Result<i32> {
    conn.execute("INSERT INTO watchitems (url) VALUES (?1)", &[url])?;
    let item_id = conn.last_insert_rowid() as i32;
    Ok(item_id)
}

fn delete_watch_item(conn: &Connection, item_id: i32) -> Result<()> {
    conn.execute("DELETE FROM watchitems WHERE id = ?1", &[&item_id])?;
    conn.execute("DELETE FROM entries WHERE item_id = ?1", &[&item_id])?;
    Ok(())
}

fn list_watch_items(conn: &Connection) -> Result<Vec<WatchItem>> {
    let mut stmt = conn.prepare("select url from watchitems")?;
    let watchitems = stmt.query_map([], |row| {
        Ok(WatchItem {
            url: row.get(0)?,
        })
    })?.map(|it| it.unwrap() ).collect::<Vec<WatchItem>>(); // TODO: unwrap copuld panic!

    Ok(watchitems)
}

fn add_entry(conn: &Connection, price: f64, created_at: i64, item_id: i32) -> Result<()> {
    conn.execute(
        "INSERT INTO entries (price, created_at, item_id) VALUES (?1, ?2, ?3)",
        params![&price, &created_at, &item_id],
    )?;
    Ok(())
}

fn get_entries(conn: &Connection) -> Result<EntryMap> {
    struct StmtEntry {
        price: f64,
        created_at: i64,
        url: String,
    }

    let mut stmt = conn.prepare(
        "select E.price, E.created_at, W.url from entries as E
        join watchitems as W on W.id=E.item_id
        order by E.created_at 
        group by W.id"
    )?;

    let entry_iter = stmt.query_map([],|row| {
        Ok(StmtEntry {
            price: row.get(0)?,
            created_at: row.get(1)?,
            url: row.get(2)?,
        })
    })?;

    let mut entries_map: EntryMap = HashMap::new();
    for entry_result in entry_iter {
        let entry = entry_result?;
        entries_map.entry(entry.url).or_default().push(Entry {
            price: entry.price,
            created_at: entry.created_at,
        })
    }

    Ok(entries_map)
}


#[cfg(test)]
mod tests {
    use super::*;
    use rusqlite::Connection;

    #[test]
    fn test_add_watch_item() {
        let conn = Connection::open_in_memory().unwrap();
        create_tables(&conn).unwrap();

        let item_id = add_watch_item(&conn, "https://example.com/item1").unwrap();
        assert_eq!(item_id, 1);
    }

    #[test]
    fn test_delete_watch_item() {
        let conn = Connection::open_in_memory().unwrap();
        create_tables(&conn).unwrap();

        add_watch_item(&conn, "https://example.com/item1").unwrap();
        add_watch_item(&conn, "https://example.com/item2").unwrap();

        delete_watch_item(&conn, 1).unwrap();

        let count: i32 = conn
            .query_row("SELECT COUNT(*) FROM watchitems", [], |row| row.get(0))
            .unwrap();
        assert_eq!(count, 1);
    }

    #[test]
    fn test_add_entry() {
        let conn = Connection::open_in_memory().unwrap();
        create_tables(&conn).unwrap();

        let item_id = add_watch_item(&conn, "https://example.com/item1").unwrap();
        add_entry(&conn, 19.99, 1621000000, item_id).unwrap();

        let count: i32 = conn
            .query_row("SELECT COUNT(*) FROM entries", [], |row| row.get(0))
            .unwrap();
        assert_eq!(count, 1);
    }

    #[test]
    fn test_get_entries() {
        let conn = Connection::open_in_memory().unwrap();
        create_tables(&conn).unwrap();

        let item_id = add_watch_item(&conn, "https://example.com/item1").unwrap();
        add_entry(&conn, 10.99, 1621000000, item_id).unwrap();

        let entry_map = get_entries(&conn).unwrap();

        // Verify the result
        let entries = entry_map.get("https://example.com/item1");
        assert_eq!(entries.is_some(), true);

        let entries_vec = entries.unwrap();
        assert_eq!(entries_vec.len(), 1);

        let entry = &entries_vec[0];
        assert_eq!(entry.price, 19.99);
        assert_eq!(entry.created_at, 1621000000);
    }

    #[test]
    fn test_list_watch_items() {
        let conn = Connection::open_in_memory().unwrap();
        create_tables(&conn).unwrap();

        add_watch_item(&conn, "https://example.com/item1").unwrap();
        add_watch_item(&conn, "https://example.com/item2").unwrap();

        let watch_items = list_watch_items(&conn).unwrap();

        // Verify the result
        assert_eq!(watch_items.len(), 2);

        let watch_item_1 = watch_items.get(0).unwrap();
        assert_eq!(watch_item_1.url, "https://example.com/item1");

        let watch_item_2 = watch_items.get(1).unwrap();
        assert_eq!(watch_item_2.url, "https://example.com/item2");
    }
}
