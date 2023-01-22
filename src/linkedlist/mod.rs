pub struct LinkedList {
    head: Option<Box<ListNode>>,
}

struct ListNode {
    val: i32,
    next: Option<Box<ListNode>>,
}

impl LinkedList {
    pub fn new() -> Self {
        Self {
            head: None,
        }
    }

    pub fn push_back(&mut self, val: i32) {
        eprintln!("Pushing {}", val);
        let mut curr_node = &mut self.head;
        loop {
            match curr_node {
                None => {
                    // create new node at end of list
                    *curr_node = Some(
                        Box::new(ListNode::new(val))
                    );
                    return;
                }
                Some(node) => {
                    // update reference to next node
                    curr_node = &mut node.next;
                }
            }
        }
    }

    pub fn pop_back(&mut self) -> Option<i32> {
        Self::pop_back_link(&mut self.head)
    }

    pub fn push_front(&mut self, val: i32) {
        eprintln!("Pushing {}", val);
        let mut new_node = Box::new(ListNode::new(val));
        // replace self.head to None for exception safety
        new_node.next = std::mem::replace(&mut self.head, None);
        self.head = Some(new_node);
    }

    pub fn pop_front(&mut self) -> Option<i32> {
        match std::mem::replace(&mut self.head, None) {
            None => {
                eprintln!("Nothing to pop!");
                None
            }

            Some(front_node) => {
                let val = front_node.val;
                self.head = front_node.next;
                Some(val)
            }
        }
    }

    fn pop_back_link(curr_node: &mut Option<Box<ListNode>>) -> Option<i32> {
        // check if there is a list to traverse
        match curr_node {
            // no node to pop
            None => {
                eprintln!("Nothing to pop!");
                None
            }

            // traverse to end of list
            Some(node) => {
                match &node.next {
                    None => {
                        let val: i32 = node.val;
                        *curr_node = None; // delete current node
                        Some(val)
                    }
                    Some(_) => {
                        // recursive call
                        Self::pop_back_link(&mut node.next)
                    }
                }
            }
        }
    }
}

impl Default for LinkedList {
    fn default() -> Self {
        Self {
            head: None,
        }
    }
}

impl ListNode {
    fn new(val: i32) -> Self {
        Self {
            val: val,
            next: None,
        }
    }
}

impl std::fmt::Display for LinkedList {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let mut curr_node = &self.head;
        let mut s: String = String::from("");

        loop {
            match &curr_node {
                None => break,

                Some(node) => {
                    s.push_str(format!("{} -> ", &node.val).as_str());
                    curr_node = &node.next;
                }
            }
        }

        s.push_str("Nil");
        write!(f, "{}", s)
    }
}
