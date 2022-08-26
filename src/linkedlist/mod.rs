pub struct LinkedList {
    head: NodeLink,
    len: u32,
}

enum NodeLink {
    None,
    ListNode { node: Box<ListNode> },
}

struct ListNode {
    val: i32,
    next: NodeLink,
}

impl LinkedList {
    pub fn new() -> Self {
        Self {
            head: NodeLink::None,
            len: 0,
        }
    }

    pub fn push(&mut self, val: i32) {
        eprintln!("Pushing {}", val);
        // keep track of
        let mut node_link = &mut self.head;
        loop {
            match node_link {
                NodeLink::None => {
                    // create new node at end of list
                    *node_link = NodeLink::ListNode {
                        node: Box::new(ListNode::new(val)),
                    };
                    self.len += 1;
                    return;
                }
                NodeLink::ListNode { node } => {
                    // update reference to next node
                    node_link = &mut node.next;
                }
            }
        }
    }
}

impl Default for LinkedList {
    fn default() -> Self {
        Self {
            head: NodeLink::None,
            len: 0,
        }
    }
}

impl Default for NodeLink {
    fn default() -> Self {
        NodeLink::None
    }
}

impl ListNode {
    fn new(val: i32) -> Self {
        Self {
            val: val,
            next: NodeLink::None,
        }
    }
}

impl std::fmt::Display for LinkedList {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let mut node_link: &NodeLink = &self.head;
        let mut s: String = String::from("");

        loop {
            match &node_link {
                NodeLink::None => {
                    break;
                }
                NodeLink::ListNode { node } => {
                    s.push_str(format!("{} -> ", &(*node).val).as_str());
                    node_link = &(*node).next;
                }
            }
        }

        s.push_str("Nil");

        write!(f, "{}", s)
    }
}
