#![feature(type_ascription)]
#![feature(box_syntax)]
extern crate xml;
#[macro_use]
extern crate log;
extern crate env_logger;
mod node;
use node::{Node,OwnedNode};
use std::fs::File;
use std::io::BufReader;
use xml::reader::{EventReader, XmlEvent};
use xml::attribute::{Attribute, OwnedAttribute};

fn indent(size: usize) -> String {
    const INDENT: &'static str = " ";
    (0..size).map(|_| INDENT)
             .fold(String::with_capacity(size*INDENT.len()), |r, s| r + s)
}

fn main() {
	env_logger::init().unwrap();
	let (mut node, mut pnode, mut ct, mut pt, mut ppt):(OwnedNode, OwnedNode, String, String, String);
	pt="".to_string(); ct="".to_string(); ppt = "".to_string();
	node = OwnedNode { id: "".to_string(), name:"".to_string(), parent:None, othername:"".to_string(), description:"".to_string() };
	pnode = OwnedNode { id: "".to_string(), name:"".to_string(), parent:None, othername:"".to_string(), description:"".to_string() };
	
	let file = File::open("sample_4.xml").unwrap();
	let file = BufReader::new(file);
	
	let parser = EventReader::new(file);
	let mut depth = 0;
	for e in parser {
		match e {
			Ok(XmlEvent::StartElement{ref name, ref attributes, .. }) => {
				ppt = pt;
				pt = ct;
				ct = name.local_name.clone();
				if name.local_name == "NODE" {
					if name.local_name == "NODES" {
						pnode = node.clone();
					}
					
					for v in attributes.iter() {
						if v.name.local_name == "ID" {
							node =  OwnedNode::new(&*v.name.local_name, "", Some(box pnode.clone()), "", "");
						}
					}
				}
				else if name.local_name == "NODES" {
//					save();
				}
				debug!("{}+{} {:?}", indent(depth), name, attributes);
				depth += 1;
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
				debug!("{}", data);
			}
			Ok(XmlEvent::EndElement { name }) => {
				if name.local_name == "NODE" {
//					save();
				}
		
				if name.local_name == "NODES" {
					pnode = *pnode.parent.unwrap();
				}
				else if name.local_name == "NODE" {
					debug!("delete {}", node);
				}
				depth -= 1;
				debug!("{}-{}", indent(depth), name);
			}
		Err(e) => {
			debug!("Error: {}", e);
				break;
			}
			_ => {}
		}
	}
}

