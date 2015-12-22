extern crate xml;
#[macro_use]
extern crate log;
extern crate env_logger;
mod node;
use node::{Node,OwnedNode};
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
	
	let pnode = Node::new("0", "RootName", Box::new(0), "RootOtherName","RootDesc");
	let node1 = Node::new("1", "Node1Name", Box::new(&pnode), "OtherName1", "Desc1");
	let node2 = Node::new("2", "Node2Name", Box::new(&node1), "OtherName2", "Desc2");
	
	debug!("pn.name:{}", &pnode.name);
	debug!("n.name:{}", &node1.name);
	
	let p = node1.parent.clone();
	info!("node1.p.id:{} p.name:{}", &p.id, &p.name);
	let p2 = node2.parent.clone();
	debug!("node2.p.id:{} p1.p.name:{}", &p2.id, &p2.name);
	let n2pp = node2.parent.parent.clone();
	debug!("n2.p.p.id:{} n2.p.p.name:{}", &n2pp.id, &n2pp.name);
	
	debug!("{}", &node1);
	debug!("{}", &node2);
		
	
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

