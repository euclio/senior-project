use std::mem;

pub struct IntegerLinkedList {
    head: Option<Node>,
    num_nodes: usize,
}

struct Node {
    next: Option<Box<Node>>,
    element: i32
}

impl IntegerLinkedList {
    pub fn new() -> IntegerLinkedList {
        IntegerLinkedList { head: None, num_nodes: 0 }
    }

    pub fn size(&self) -> usize {
        self.num_nodes
    }

    pub fn insert(&mut self, index: usize, element: i32) {
        if index == 0 && self.num_nodes > 0 {
            let old_head = mem::replace(&mut self.head, None);
            let new_head =
                Node::new_with_tail(element, Some(Box::new(old_head.unwrap())));
            self.head = Some(new_head);
            self.num_nodes += 1;
            return
        }

        match self.head {
            Some(ref mut head) => {
                head.insert(index, element);
            },
            None => {
                if index == 0 {
                    self.head = Some(Node::new(element));
                } else {
                    panic!("list index out of bounds")
                }
            }
        }
        self.num_nodes += 1
    }

    pub fn remove(&mut self, index: usize) -> i32 {
        let val;
        match self.head {
            Some(ref mut head) => {
                val = head.remove(index);
            },
            None => panic!("list index out of bounds")
        }
        self.num_nodes -= 1;
        val
    }

    pub fn head(&self) -> i32 {
        match self.head {
            Some(ref head) => {
                head.element
            },
            None => panic!("no head of list")
        }
    }

    pub fn tail(&self) -> i32 {
        self.get(self.num_nodes - 1)
    }

    pub fn get(&self, index: usize) -> i32 {
        match self.head {
            Some(ref head) => {
                head.get(index)
            },
            None => panic!("list index out of bounds")
        }
    }
}

impl Node {
    pub fn new(element: i32) -> Node {
        Node { next: None, element: element }
    }

    pub fn new_with_tail(element: i32, next: Option<Box<Node>>) -> Node {
        Node { next: next, element: element }
    }

    pub fn insert(&mut self, index: usize, element: i32) {
        match self.next {
            Some(ref mut next) => {
                if index > 0 {
                    return next.insert(index - 1, element);
                }
            },
            None => {}
        };

        let old_node = mem::replace(&mut self.next, None);
        let mut new_node = Box::new(Node::new(element));
        new_node.next = old_node;
        self.next = Some(new_node)
    }

    pub fn remove(&mut self, index: usize) -> i32 {
        let ret;

        match self.next {
            Some(ref mut next) => {
                if index > 0 {
                    return next.remove(index - 1)
                } else {
                    ret = self.element
                }
            }
            None => if index == 0 {
                ret = self.element
            } else {
                panic!("list index out of bounds")
            }
        };

        self.next = match self.next {
            Some(ref mut next) => { mem::replace(&mut next.next, None) },
            None => None
        };

        ret
    }

    pub fn get(&self, index: usize) -> i32 {
        if index == 0 {
            self.element
        } else {
            match self.next {
                Some(ref next) => next.get(index - 1),
                None => panic!("list index out of bounds")
            }
        }
    }
}
