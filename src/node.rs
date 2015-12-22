use std::fmt;

#[derive(Clone, Eq, PartialEq, Hash, Debug)]
pub struct Node<'a, T> {
    pub id: &'a str,
    pub name: &'a str,
    pub parent: Box<T>,
    pub othername: &'a str,	
    pub description: &'a str
}

impl<'a,T> fmt::Display for Node<'a, T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "id:{} name:{} othername:{}", self.id, self.name, self.othername)
    }
}

impl<'a,T> Node<'a,T> {
    #[inline]
    pub fn  new(id: &'a str, name: &'a str, node: Box<T>, othername: &'a str, description: &'a str) -> Node<'a,T> {
        Node {
            id: id,
            name: name,
            parent: node,
            othername: othername,
            description: description
        }
    }
}


#[derive(Clone, Eq, PartialEq, Hash, Debug)]
pub struct OwnedNode {
    pub id: String,
    pub name: String,
    pub parent: Option<Box<OwnedNode>>,
    pub othername: String,
    pub description: String
}

impl OwnedNode {
    #[inline]
    pub fn new<S: Into<String>>(id: S, name: S, parent: Option<Box<OwnedNode>>, othername: S, description: S) -> OwnedNode {
        OwnedNode {
            id: id.into(),
            name: name.into(),
            parent: parent,
            othername: othername.into(),
            description: description.into()
        }
    }
}

impl fmt::Display for OwnedNode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    	let parent = self.parent.clone().unwrap();
        write!(f, "id:{} name:{} p.id:{} p.name:{} other:{} desc:{}", self.id, &*self.name, &*parent.id, &*parent.name, &*self.othername, &*self.description)
    }
}

#[cfg(test)]
mod tests {
  use node::{Node,OwnedNode};
	
	#[test]
	fn node_display() {
		let root = OwnedNode { id: "".to_string(), name:"".to_string(), parent:None, othername:"".to_string(), description:"".to_string() };
	    let pnode = OwnedNode::new("0", "PNName", Some(box root), "PNOtherName","PNDesc");
	    
	    let node = OwnedNode::new("1", "NodeName", Some(box pnode.clone()), "OtherName", "Desc");
			
			assert_eq!(node.name.clone(), "NodeName");
			assert_eq!(pnode.name.clone(), "PNName");
	}
}
