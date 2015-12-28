#![feature(box_syntax)]
extern crate xml;
#[macro_use]
extern crate log;
extern crate rustc_serialize;
pub mod node;
use node::{Node};
use std::fs::File;
use std::io::BufReader;
use xml::reader::{EventReader, XmlEvent};
use std::env;

pub trait DataStore {
	fn save(&self, node: &Node);
}

pub fn xml_walk<T: DataStore>(ds: &T) {
	
	let (mut node, mut pnode, mut ct, mut pt):(Node, Node, String, String);
	pt="".to_string(); ct="".to_string();
	node = Node { id: "".to_string(), name:"".to_string(), parent:None, othername:"".to_string(), description:"".to_string() };
	pnode = Node { id: "".to_string(), name:"".to_string(), parent:None, othername:"".to_string(), description:"".to_string() };
	
	let file = File::open(getpath()).unwrap();
	let file = BufReader::new(file);
	
	
	let parser = EventReader::new(file);
	for el in parser {
		match el {
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
					ds.save(&node);
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
					ds.save(&node);
				}
				if name.local_name == "NODES" {
//					debug!("Befor p.id:{} p.name:{}", pnode.id, pnode.name);
					pnode = *pnode.parent.clone().unwrap_or( box Node{ id: "".to_string(), name:"".to_string(), parent:None, othername:"".to_string(), description:"".to_string() } );
				}
				else if name.local_name == "NODE" {
				}
			}
		Err(el) => {
			debug!("Error: {}", el);
				break;
			}
			_ => {}
		}
	}
}

fn getpath() -> String {
    let args: Vec<String> = env::args().collect();

    match args.len() {
        1 => {
            panic!("first argument must be a path to xml file")
        },
        2 => args[1].clone(),
		n => { debug!("rusttol n args:{}", n); args[1].clone()}
    }
}


