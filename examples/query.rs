extern crate rusqlite;
extern crate time;

use time::Timespec;
use rusqlite::Connection;

#[derive(Debug)]
struct Person {
    id: i32,
    name: String,
    time_created: Timespec,
    data: Option<Vec<u8>>
}

fn main() {
    let conn = Connection::open("db/db.sqlite").unwrap();
//    let conn = Connection::open("/home/projects/rusttol/db.sqlite").unwrap();

//    conn.execute("CREATE TABLE person (
//                  id              INTEGER PRIMARY KEY,
//                  name            TEXT NOT NULL,
//                  time_created    TEXT NOT NULL,
//                  data            BLOB
//                  )", &[]).unwrap();
//
    conn.execute("create table tol (
		id integer primary key,
		name text,
		parent integer,
		othername text,
		description text
		)", &[]).unwrap();
//    let me = Person {
//        id: 1,
//        name: "Brahma Deve".to_string(),
//        time_created: time::get_time(),
//        data: Some(vec!(3))
//    };
//    conn.execute("INSERT INTO person (name, time_created, data)
//                  VALUES ($1, $2, $3)",
//                 &[&me.name, &me.time_created, &me.data]).unwrap();

    let mut stmt = conn.prepare("SELECT id, name, time_created, data FROM person").unwrap();
    let person_iter = stmt.query_map(&[], |row| {
        Person {
            id: row.get(0),
            name: row.get(1),
            time_created: row.get(2),
            data: row.get(3)
        }
    }).unwrap();

    for person in person_iter {
        println!("Found person {:?}", person.unwrap());
    }
}