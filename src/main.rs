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
//type E<'a> = Node<'a,Node<'a, String>>;
type E<'a> = String;
use xml::reader::{EventReader, XmlEvent};
use xml::attribute::{Attribute, OwnedAttribute};

fn indent(size: usize) -> String {
    const INDENT: &'static str = " ";
    (0..size).map(|_| INDENT)
             .fold(String::with_capacity(size*INDENT.len()), |r, s| r + s)
}

fn main() {
	env_logger::init().unwrap();
	let (mut node, mut pnode, mut ct, mut pt, mut ppt):(OwnedNode<E>, OwnedNode<E>, String, String, String);
	pt="".to_string(); ct="".to_string(); ppt = "".to_string();
//	pnode = OwnedNode::new("","", box OwnedNode::new("", "", box OwnedNode::new("","",box "".to_string(),"",""),"",""),"","");
	node = OwnedNode::new("","", box 0.to_string(),"","");
	pnode = OwnedNode::new("","", box 0.to_string(),"","");
	
	let file = File::open("sample_4.xml").unwrap();
	let file = BufReader::new(file);
//	let pnode = Node::new("0", "RootName", Box::new(0), "RootOtherName","RootDesc");
//	let node1 = Node::new("1", "Node1Name", Box::new(&pnode), "OtherName1", "Desc1");
//	let node2 = Node::new("2", "Node2Name", Box::new(&node1), "OtherName2", "Desc2");
//	
//	debug!("pn.name:{}", &pnode.name);
//	debug!("n.name:{}", &node1.name);
//	
//	let p = node1.parent.clone();
//	info!("node1.p.id:{} p.name:{}", &p.id, &p.name);
//	let p2 = node2.parent.clone();
//	debug!("node2.p.id:{} p1.p.name:{}", &p2.id, &p2.name);
//	let n2pp = node2.parent.parent.clone();
//	debug!("n2.p.p.id:{} n2.p.p.name:{}", &n2pp.id, &n2pp.name);
//	
//	debug!("{}", &node1);
//	debug!("{}", &node2);
		
	
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
//						OwnedAttribute{ref name, ref value} => 
							let node =  OwnedNode::new(&*v.name.local_name, "", box pnode.clone(), "", "");
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

