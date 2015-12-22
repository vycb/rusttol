extern crate xml;
#[macro_use]
extern crate log;
extern crate env_logger;
mod node;
use std::fs::File;
use std::io::BufReader;

use xml::reader::{EventReader, XmlEvent};

fn indent(size: usize) -> String {
    const INDENT: &'static str = "    ";
    (0..size).map(|_| INDENT)
             .fold(String::with_capacity(size*INDENT.len()), |r, s| r + s)
}

fn main() {
	env_logger::init().unwrap();
	let (mut ct, mut pt, mut ppt):(String, String, String);
	let file = File::open("sample_4.xml").unwrap();
	let file = BufReader::new(file);
	
	let parser = EventReader::new(file);
	let mut depth = 0;
	for e in parser {
		match e {
			Ok(XmlEvent::StartElement{ref name, ref attributes, .. }) => {
//				ppt = pt;
//				pt = ct;
//				ct = tag;
//				if(tag.name === "NODE"){
//					if(pt.name === "NODES"){
//						pnode = node;
//					}
//					node = new Node();
//					node.p = pnode;
//					node.id = tag.attributes["ID"];
//				}
//				else if(tag.name === "NODES"){
//					save();
//				}
				debug!("{}+{}", indent(depth), name);
			depth += 1;
		}
		Ok(XmlEvent::EndElement { name }) => {
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

