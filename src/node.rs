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
pub struct OwnedNode<T> {
    /// Node name.
    pub id: String,
    pub name: String,
    pub parent: Box<T>,
    pub othername: String,
    pub description: String
}

impl <T>OwnedNode<T> {
    #[inline]
    pub fn new<S: Into<String>>(id: S, name: S, parent: Box<T>, othername: S, description: S) -> OwnedNode<T> {
        OwnedNode {
            id: id.into(),
            name: name.into(),
            parent: parent,
            othername: othername.into(),
            description: description.into()
        }
    }
}

impl <T>fmt::Display for OwnedNode<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//    	let parent = self.parent;
        write!(f, "id:{} name:{} other:{} desc:{}", self.id, &*self.name,/* &*self.parent.id, &*self.parent.name,*/ &*self.othername, &*self.description)
    }
}

#[cfg(test)]
mod tests {
  use node::{Node,OwnedNode};
	
	#[test]
	fn node_display() {
	    let pnode = OwnedNode::new("0", "PNName", Box::new(0), "PNOtherName","PNDesc");
	    
	    let node = OwnedNode::new("1", "NodeName", Box::new(&pnode), "OtherName", "Desc");
			
			assert_eq!(node.name, "NodeName");
			assert_eq!(pnode.name, "PNName");
	}
}
