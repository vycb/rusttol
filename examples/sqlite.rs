#![feature(box_syntax)]
extern crate rusqlite;
extern crate rusttol;
use rusttol::node::{Node};
use rusqlite::Connection;
#[macro_use]
extern crate log;
extern crate env_logger;
use std::env;

fn main(){
	env_logger::init().unwrap();
	
	let sqlc = SqlClient::new();
	
	let args: Vec<String> = env::args().collect();
	
	match args.len() {
        3 => if args[1] == "query" { sqlc.query( &args[2].clone() ) }
        	 else if args[1] == "toroot" { sqlc.toroot( &args[2].clone() ) },
		_ => 
			rusttol::xml_walk(&sqlc)
    } 
	
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

	pub fn query(&self, key: &str) {
	    let mut stmt = self.conn.prepare("SELECT t.id, t.name, t.parent, t.othername, t.description, (SELECT count(*) from tol WHERE parent=t.id) as childs FROM tol t WHERE (t.name LIKE $1 OR t.othername LIKE $1 OR t.description LIKE $1) ORDER BY childs desc, name asc").unwrap();
	    let node_iter = stmt.query_map(&[&key], |row| {
	        QNode {
	            id: row.get(0),
	            name: row.get(1),
	            parent:row.get(2),
	            othername: row.get(3),
	            description: row.get(4),
	            childs: row.get(5)
	        }
	    }).unwrap();
	
	    for node in node_iter {
	    	let n = node.unwrap();
	        println!("{} {} p:{} o:{} d:{} c:{}", &n.id, &n.name, n.parent, &n.othername, &n.description, &n.childs);
	    }
	}

	pub fn toroot(&self, key: &str) {
	    let mut stmt = self.conn.prepare("SELECT t.id, t.name, t.parent, t.othername, t.description, (SELECT count(*) from tol WHERE parent=t.id) as childs FROM tol t WHERE t.name LIKE $1 ORDER BY childs desc, name asc LIMIT 1").unwrap();
	    let node_iter = stmt.query_map(&[&key], |row| {
	        QNode {
	            id: row.get(0),
	            name: row.get(1),
	            parent:row.get(2),
	            othername: row.get(3),
	            description: row.get(4),
	            childs: row.get(5)
	        }
	    }).unwrap();
	
	    for node in node_iter {
	    	let n = node.unwrap();
	        println!("{} {} p:{} o:{} d:{} c:{}", &n.id, &n.name, n.parent, &n.othername, &n.description, &n.childs);
	        
	        self.get_parent(&n.parent)
	    }
	}

	pub fn get_parent(&self, id: &i32) {
		 let mut stmt = self.conn.prepare("select t.id, t.name, t.parent, t.othername, t.description, (SELECT count(*) from tol WHERE parent=t.id) as childs from tol t where id=$1").unwrap();
	    let mut rows = stmt.query(&[id]).unwrap();
	    
	    let row = rows.next().unwrap().unwrap();
	    println!("{} {} p:{} o:{} d:{} c:{}", &row.get::<i32>(0), &row.get::<String>(1), &row.get::<i32>(2), &row.get::<String>(3), &row.get::<String>(4), &row.get::<i32>(5));
	    
	    if row.get::<i32>(2) > 0 {
	    	self.get_parent(&row.get::<i32>(2));
	    }
	}
}

#[derive(Debug)]
struct QNode{
	id: i32,
	name: String,
	parent: i32,
	othername: String,
	description: String,
	childs: i32
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