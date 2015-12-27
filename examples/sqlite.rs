extern crate rusqlite;
extern crate rusttol;
//use rusttol::*;
use rusttol::node::{Node};
use rusqlite::Connection;
#[macro_use]
extern crate log;
extern crate env_logger;

fn main(){
	env_logger::init().unwrap();
	
	let sqlc = SqlClient::new();
	
	rusttol::xml_walk(&sqlc);
}

struct SqlClient {
	conn: Connection
}


impl SqlClient {
	
	pub fn new() -> SqlClient {
		SqlClient{
			conn: Connection::open("db/db.sqlite").unwrap_or_else(|e|{ panic!("Connection error:{}", e) } )
		}
	}
}

impl rusttol::DataStore for SqlClient {
	
	fn save(&self, node: &Node) {
		
		let parent = node.parent.clone().unwrap();
		
	    debug!("Save id:{} name:{} p.id:{} p.name:{} oname:{} desc:{}",
	                 &node.id, &node.name, &parent.id, &parent.name, &node.othername, &node.description);
	    
	    self.conn.execute("INSERT OR IGNORE INTO tol (id, name, parent, othername, description)
	                  VALUES ($1, $2, $3, $4, $5)",
	                 &[&node.id.to_string(), &node.name.to_string(), &parent.id.to_string(), &node.othername.to_string(), &node.description.to_string()]).unwrap_or_else(|e|{ panic!("INSERT tol error:{}", e) });
	    
	}
}