#![feature(type_ascription)]
#![feature(box_syntax)]
extern crate xml;
extern crate rusqlite;
#[macro_use]
extern crate log;
extern crate env_logger;
mod node;
use node::{Node};
use std::fs::File;
use std::io::BufReader;
use xml::reader::{EventReader, XmlEvent};
use rusqlite::Connection;

fn main() {
	env_logger::init().unwrap();
	let (mut node, mut pnode, mut ct, mut pt):(Node, Node, String, String);
	pt="".to_string(); ct="".to_string();
	node = Node { id: "".to_string(), name:"".to_string(), parent:None, othername:"".to_string(), description:"".to_string() };
	pnode = Node { id: "".to_string(), name:"".to_string(), parent:None, othername:"".to_string(), description:"".to_string() };
	
	let file = File::open("tol.xml").unwrap();
	let file = BufReader::new(file);
	
	let parser = EventReader::new(file);
	for e in parser {
		match e {
			Ok(XmlEvent::StartElement{ref name, ref attributes, .. }) => {
				pt = ct;
				ct = name.local_name.clone();
				
				if name.local_name == "NODE" {
					if pt == "NODES" {
						pnode = node.clone();
//						debug!("Clone n.id:{} n.name:{} p.id:{} p.name:{}", node.id, node.name, pnode.id, pnode.name);
					}
					
					for v in attributes.iter() {
						if v.name.local_name == "ID" {
//							debug!("New p.id:{} p.name:{}", pnode.id, pnode.name);
							node =  Node::new(&*v.value, "", Some(box pnode.clone()), "", "");
						}
					}
				}
				else if name.local_name == "NODES" {
					save(&node);
				}
			}
			Ok(XmlEvent::CData(ref data) ) => {
				if ct == "NAME" && pt == "NODE"{
					node.name = data.clone();
				}
				else if ct == "DESCRIPTION"{
					node.description = data.to_owned();
				}
				else if ct == "NAME" && pt == "OTHERNAME"{
					node.othername = node.othername.clone() + if !node.othername.clone().is_empty() { ", "} else { "" } + &data;
				}
			}
			Ok(XmlEvent::EndElement { name }) => {
				if name.local_name == "NODE" {
					save(&node);
				}
				if name.local_name == "NODES" {
//					debug!("Befor p.id:{} p.name:{}", pnode.id, pnode.name);
					pnode = *pnode.parent.clone().unwrap_or( box Node{ id: "".to_string(), name:"".to_string(), parent:None, othername:"".to_string(), description:"".to_string() } );
				}
				else if name.local_name == "NODE" {
				}
			}
		Err(e) => {
			debug!("Error: {}", e);
				break;
			}
			_ => {}
		}
	}
}


fn save(node: &Node)
{
	let conn = Connection::open("db/db.sqlite").unwrap_or_else(|e|{ panic!("Connection error:{}", e) } );
	let parent = node.parent.clone().unwrap();
	
    debug!("Save id:{} name:{} p.id:{} p.name:{} oname:{} desc:{}",
                 &node.id, &node.name, &parent.id, &parent.name, &node.othername, &node.description);
    
    conn.execute("INSERT OR IGNORE INTO tol (id, name, parent, othername, description)
                  VALUES ($1, $2, $3, $4, $5)",
                 &[&node.id.to_string(), &node.name.to_string(), &parent.id.to_string(), &node.othername.to_string(), &node.description.to_string()]).unwrap_or_else(|e|{ panic!("INSERT tol error:{}", e) });
    
}
