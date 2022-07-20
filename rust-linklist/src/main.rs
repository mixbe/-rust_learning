// 链表节点
#[derive(Debug)]
struct Node {
    value: i32,
    next: Option<Box<Node>>,
}

impl Node {
    pub fn new(value: i32) -> Box<Node> {
        Box::new(Node { value, next: None })
    }

    pub fn link(&mut self, node: Box<Node>) {
        self.next = Some(node);
    }
}

#[derive(Debug)]
struct BoxedLinkedList {
    head: Option<Box<Node>>,
}

impl BoxedLinkedList {
    pub fn new() -> Self {
        Self { head: None }
    }

    pub fn is_empty(&self) -> bool {
        self.head.is_none()
    }
    // 头插入
    pub fn push_front(&mut self, value: i32) {
        let mut new_node = Node::new(value);
        if let Some(node) = self.head.take() {
            new_node.link(node);
        }
        self.head = Some(new_node);
    }


    pub fn pop_front(&mut self) -> Option<i32> {
        match self.head.take() {
            None => None,
            Some(curr) => {
                self.head = curr.next;
                Some(curr.value)
            }
        }
    }

    pub fn push_back(&mut self, value: i32) {
        let mut new_node = Node::new(value);
        match self.head.as_mut() {
            None => self.head = Some(new_node),
            Some(mut curr) => {
                while curr.next.is_some() {
                    curr = curr.next.as_mut().unwrap();
                }
                curr.link(new_node);
            }
        }
    }
    pub fn pop_back(&mut self) -> Option<i32> {
        match self.head.as_mut() {
            None => None,
            Some(mut curr) => {
                while curr.next.is_some() && curr.next.as_ref().unwrap().next.is_some() {
                    curr = curr.next.as_mut().unwrap();
                }
                match curr.next {
                    Some(_) => Some(curr.next.take().unwrap().value),
                    None => Some(self.head.take().unwrap().value)
                }
            }
        }
    }

    // 从小到大的顺序插入
    fn insert(&mut self, value: i32) {
        let mut new_node = Node::new(value);
        match self.head.as_mut() {
            None => self.head = Some(new_node),
            Some(mut curr) => {
                if value <= curr.value {
                    self.push_front(value);
                } else {
                    while curr.next.is_some() && curr.next.as_ref().unwrap().value < value {
                        curr = curr.next.as_mut().unwrap();
                    }
                    if let Some(node) = curr.next.take() {
                        new_node.link(node);
                    }
                    curr.next = Some(new_node);
                }
            }
        }
    }

    // 删除
    fn delete(&mut self, value: i32) -> bool {
        match self.head.as_mut() {
            None => false,
            Some(mut curr) => {
                if curr.value == value {
                    self.head = self.head.take().unwrap().next;
                    true
                } else {
                    while curr.next.is_some() && curr.next.as_ref().unwrap().value != value {
                        curr = curr.next.as_mut().unwrap();
                    }
                    match curr.next.take() {
                        None => false,
                        Some(node) => {
                            curr.next = node.next;
                            true
                        }
                    }
                }
            }
        }
    }

    // 反转
    fn reverse(&mut self) {
        if self.is_empty() || self.head.as_ref().unwrap().next.is_none() {
            return;
        }
        let mut left = self.head.as_mut().unwrap().next.take();
        while left.is_some() {
            let mut t = left.take().unwrap();
            left = t.next;
            t.next = self.head.take();
            self.head = Some(t)
        }
    }
}


fn main() {
    let mut linklist = BoxedLinkedList::new();
    linklist.push_back(1);
    linklist.push_back(2);
    linklist.push_back(4);
    println!("{:?}", linklist);

    linklist.insert(3);
    println!("{:?}", linklist);

    linklist.delete(3);
    println!("{:?}", linklist);

    linklist.reverse();
    println!("{:?}", linklist);


    // let value = linklist.pop_back();
    // println!("{}", value.unwrap());
    // println!("{:?}", linklist)
}
