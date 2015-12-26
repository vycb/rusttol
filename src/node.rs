use std::fmt;


#[derive(Clone, Eq, PartialEq, Hash, Debug)]
pub struct Node {
    pub id: String,
    pub name: String,
    pub parent: Option<Box<Node>>,
    pub othername: String,
    pub description: String
}

impl Node {
    #[inline]
    pub fn new<S: Into<String>>(id: S, name: S, parent: Option<Box<Node>>, othername: S, description: S) -> Node {
        Node {
            id: id.into(),
            name: name.into(),
            parent: parent,
            othername: othername.into(),
            description: description.into()
        }
    }
}

impl fmt::Display for Node {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    	let parent = self.parent.clone().unwrap();
        write!(f, "id:{} name:{} p.id:{} p.name:{} other:{} desc:{}", self.id, &*self.name, &*parent.id, &*parent.name, &*self.othername, &*self.description)
    }
}

#[cfg(test)]
mod tests {
  use node::{Node};
	
	#[test]
	fn node_display() {
		let root = Node { id: "".to_string(), name:"".to_string(), parent:None, othername:"".to_string(), description:"".to_string() };
	    let pnode = Node::new("0", "PNName", Some(box root), "PNOtherName","PNDesc");
	    
	    let node = Node::new("1", "NodeName", Some(box pnode.clone()), "OtherName", "Desc");
			
			assert_eq!(node.name.clone(), "NodeName");
			assert_eq!(pnode.name.clone(), "PNName");
	}
}
