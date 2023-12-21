use crate::Target;

pub(crate) struct Root {
    nodes: Vec<(String, Vec<Node>)>,
    last_node: Option<*mut Node>,
}

impl Root {
    pub(crate) fn new() -> Self {
        Root {
            nodes: Vec::new(),
            last_node: None,
        }
    }

    pub(crate) fn add(&mut self, node: Node) {
        if node.offset == 0 {
            let k = node.key.to_string();
            if let Some(n) = self.nodes.iter_mut().find(|n| n.0 == k) {
                n.1.push(node);
                self.last_node = n.1.last_mut().map(|n| n as *mut Node);
            } else {
                self.nodes.push((k, vec![node]));
                if let Some(n) = self.nodes.last_mut() {
                    self.last_node = n.1.last_mut().map(|n| n as *mut Node);
                }
            }
        } else {
            unsafe {
                if let Some(p) = self.last_node {
                    if node.offset == (*p).offset + 1 {
                        self.last_node = (*p).add(node);
                    } else if let Some(p) = (*p).get_parent_by_offset(node.offset) {
                        self.last_node = (*p).add(node);
                    }
                }
            }
        }
    }

    pub(crate) fn build(&self) -> Vec<Target> {
        build(&self.nodes)
    }
}

fn build(nodes: &Vec<(String, Vec<Node>)>) -> Vec<Target> {
    let mut rst = Vec::new();
    for v in nodes {
        let mut o = Target::new(v.0.to_string());
        for n in &v.1 {
            if n.text.len() > 0 {
                o.text.push(n.text.clone());
            }
            o.value.append(&mut build(&n.nodes));
        }
        rst.push(o);
    }
    rst
}

#[derive(Debug)]
pub(crate) struct Node {
    parent: Option<*mut Node>,
    nodes: Vec<(String, Vec<Node>)>,
    offset: usize,
    key: String,
    text: String,
}

impl Node {
    pub(crate) fn new(offset: usize, key: String, text: String) -> Self {
        Node {
            parent: None,
            nodes: Vec::new(),
            offset,
            key,
            text,
        }
    }

    fn add(&mut self, mut node: Node) -> Option<*mut Node> {
        node.parent = Some(self);
        let k = node.key.to_string();
        if let Some(n) = self.nodes.iter_mut().find(|n| n.0 == k) {
            n.1.push(node);
            n.1.last_mut().map(|n| n as *mut Node)
        } else {
            self.nodes.push((k, vec![node]));
            self.nodes.last_mut()?.1.last_mut().map(|n| n as *mut Node)
        }
    }

    fn get_parent_by_offset(&mut self, seekoffset: usize) -> Option<*mut Node> {
        if self.offset == seekoffset {
            return self.parent;
        } else if seekoffset < self.offset {
            if let Some(p) = self.parent {
                unsafe {
                    return (*p).get_parent_by_offset(seekoffset);
                }
            }
        }
        None
    }
}
