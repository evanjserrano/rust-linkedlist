pub struct LinkedList {
    head: Option<Box<ListNode>>,
    len: u32,
}

struct ListNode {
    val: i32,
    next: Option<Box<ListNode>>,
}

impl LinkedList {
    pub fn new() -> Self {
        Self {
            head: None,
            len: 0,
        }
    }

    pub fn push_back(&mut self, val: i32) {
        eprintln!("Pushing {}", val);
        // keep track of
        let mut curr_node = &mut self.head;
        loop {
            match curr_node {
                None => {
                    // create new node at end of list
                    *curr_node = Some(
                        Box::new(ListNode::new(val))
                    );
                    self.len += 1;
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

    fn pop_back_link(curr_node: &mut Option<Box<ListNode>>) -> Option<i32> {
        // ... -> a -> {node} -> Nil
        //     root -> {node} -> Nil
        //             {root} -> Nil
        //                       Nil

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
            len: 0,
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
