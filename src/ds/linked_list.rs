use std::cell::{Ref, RefCell};
use std::rc::{Rc, Weak};

pub type WeakLink<T> = Option<Weak<RefCell<Node<T>>>>;
pub type Link<T> = Option<Rc<RefCell<Node<T>>>>;

#[derive(Debug)]
pub struct Node<T> {
    pub data: T,
    pub prev: WeakLink<T>,
    pub next: Link<T>,
}

#[derive(Debug)]
pub struct SinglyLinkedList<T: Copy> {
    head: Link<T>,
}

impl<T: Copy> SinglyLinkedList<T> {
    pub fn new() -> Self {
        SinglyLinkedList { head: None }
    }
    pub fn peek(&self) -> Option<Rc<RefCell<Node<T>>>> {
        self.head.clone()
    }
    pub fn append(&mut self, val: T) {
        let new_node = Rc::new(RefCell::new(Node {
            data: val,
            prev: None,
            next: None,
        }));
        if let Some(node) = self.head.as_mut() {
            let mut curr = node.clone();
            while !curr.borrow().next.is_none() {
                let next = curr.borrow().next.clone();
                curr = next.unwrap();
            }
            curr.borrow_mut().next = Some(new_node);
        } else {
            self.head = Some(new_node);
        }
    }
    pub fn append_node(&mut self, new_node: Rc<RefCell<Node<T>>>) {
        if let Some(node) = self.head.as_mut() {
            let mut curr = node.clone();
            while !curr.borrow().next.is_none() {
                let next = curr.borrow().next.clone();
                curr = next.unwrap();
            }
            curr.borrow_mut().next = Some(new_node);
        } else {
            self.head = Some(new_node);
        }
    }
    pub fn insert(&mut self, val: T, index: usize) {
        let new_node = Rc::new(RefCell::new(Node {
            data: val,
            prev: None,
            next: None,
        }));
        let mut count = 0;
        if let Some(node) = self.head.as_mut() {
            if index == 0 {
                new_node.borrow_mut().next = self.head.take();
                self.head = Some(new_node);
                return;
            }
            let mut curr = node.clone();
            let mut prev = node.clone();
            while count < index {
                count += 1;
                prev = curr.clone();
                let next = curr.borrow().next.clone();
                if next.is_none() {
                    self.append(val);
                    return;
                }
                curr = next.unwrap();
            }
            prev.borrow_mut().next = Some(new_node.clone());
            new_node.borrow_mut().next = Some(curr);
        } else {
            self.head = Some(new_node)
        }
    }
    pub fn pop(&mut self) -> Option<T> {
        if let Some(node) = self.head.as_mut() {
            let mut curr = node.clone();
            let mut prev = curr.clone();
            while !curr.borrow().next.is_none() {
                prev = curr.clone();
                let next = curr.borrow().next.clone();
                curr = next.unwrap();
            }
            let out = prev.borrow_mut().next.take();
            return Some(out.unwrap().borrow().data);
        }
        None
    }
    pub fn remove_kth_last(&mut self, index: usize) -> Option<T> {
        if index == 0 {
            return None;
        }
        if let Some(node) = self.head.as_mut() {
            let mut slow = node.clone();
            let mut fast = node.clone();
            let mut count = 0;
            while count < index {
                let next = fast.borrow().next.clone();
                if let Some(node) = next {
                    fast = node;
                } else {
                    return None;
                }
                count += 1;
            }
            while !fast.borrow().next.is_none() {
                let slow_next = slow.borrow().next.clone();
                slow = slow_next.unwrap();
                let fast_next = fast.borrow().next.clone();
                fast = fast_next.unwrap();
            }
            let mut slow_next = slow.borrow().next.clone();
            let slow_next_next = slow_next.clone().unwrap().borrow().next.clone();
            slow.borrow_mut().next = slow_next_next;
            let out = slow_next.take();
            return Some(out.unwrap().borrow().data);
        }
        None
    }
}

#[derive(Debug)]
pub struct DoublyLinkedList<T> {
    head: Link<T>,
    tail: Link<T>,
}

impl<T: Copy> DoublyLinkedList<T> {
    pub fn new() -> Self {
        Self {
            head: None,
            tail: None,
        }
    }
    pub fn append(&mut self, val: T) {
        let new_node = Some(Rc::new(RefCell::new(Node {
            data: val,
            prev: None,
            next: None,
        })));
        if let Some(node) = self.tail.as_ref() {
            new_node.as_deref().unwrap().borrow_mut().prev = Some(Rc::downgrade(node));
        }
        if let Some(node) = self.tail.as_ref() {
            node.borrow_mut().next = new_node.clone();
            self.tail = new_node
        } else {
            self.tail = new_node;
            if self.head.is_none() {
                self.head = self.tail.clone()
            }
        }
    }
    pub fn append_node(&mut self, new_node: Rc<RefCell<Node<T>>>) {
        if let Some(node) = self.tail.as_ref() {
            new_node.borrow_mut().prev = Some(Rc::downgrade(node));
        }
        if let Some(node) = self.tail.as_ref() {
            node.borrow_mut().next = Some(new_node.clone());
            self.tail = Some(new_node)
        } else {
            self.tail = Some(new_node);
            if self.head.is_none() {
                self.head = self.tail.clone()
            }
        }
    }
    pub fn pop_head(&mut self) -> Option<Rc<RefCell<Node<T>>>> {
        if let Some(node) = self.head.take() {
            self.head = node.borrow().next.clone();
            let out = node;
            return Some(out);
        }
        None
    }
    pub fn pop_tail(&mut self) -> Option<Rc<RefCell<Node<T>>>> {
        if let Some(node) = self.tail.take() {
            let mut prev = node.borrow().prev.clone();
            prev.as_mut().unwrap().upgrade().unwrap().borrow_mut().next = None;
            let out = node;
            return Some(out);
        }
        None
    }
    pub fn peek_head(&self) -> Option<Rc<RefCell<Node<T>>>> {
        self.head.clone()
    }
    pub fn peek_tail(&self) -> Option<Rc<RefCell<Node<T>>>> {
        self.tail.clone()
    }
    pub fn remove_node(&mut self, node: Rc<RefCell<Node<T>>>) {
        let prev = node.borrow().prev.as_ref().unwrap().upgrade();
        let next = node.borrow().next.clone();
        if prev.is_some() {
            prev.clone().unwrap().borrow_mut().next = next.clone();
        } else {
            self.head = next.clone()
        }
        if next.is_some() {
            if prev.is_some() {
                next.unwrap().borrow_mut().prev = Some(Rc::downgrade(&prev.unwrap()));
            } else {
                self.head = next.clone()
            }
        } else {
            self.tail = prev.clone()
        }
        node.borrow_mut().prev = None;
        node.borrow_mut().next = None;
    }
}

mod tests {
    use super::*;
    #[test]
    fn test_singly_linked_list_append() {
        let mut linked_list = SinglyLinkedList::new();
        linked_list.append(10);
        linked_list.append(20);
        linked_list.append(30);
        linked_list.append(40);
        println!("{:?}", linked_list);
    }
    #[test]
    fn test_singly_linked_list_pop() {
        let mut linked_list = SinglyLinkedList::new();
        linked_list.append(10);
        linked_list.append(20);
        linked_list.append(30);
        linked_list.append(40);
        linked_list.pop();
        println!("{:?}", linked_list);
    }
    #[test]
    fn test_singly_linked_list_insert() {
        let mut linked_list = SinglyLinkedList::new();
        linked_list.insert(10, 10);
        linked_list.append(20);
        linked_list.append(30);
        linked_list.append(40);
        linked_list.insert(5, 1);
        linked_list.insert(50, 50);
        linked_list.insert(25, 3);
        println!("{:?}", linked_list);
    }
    #[test]
    fn test_singly_linked_list_remove_kth_last() {
        let mut linked_list = SinglyLinkedList::new();
        linked_list.insert(10, 10);
        linked_list.append(20);
        linked_list.append(30);
        linked_list.append(40);
        linked_list.insert(5, 1);
        linked_list.insert(50, 50);
        linked_list.insert(25, 3);
        linked_list.remove_kth_last(0);
        println!("{:?}", linked_list);
    }

    #[test]
    fn test_doubly_linked_list_append() {
        let mut linked_list = DoublyLinkedList::new();
        linked_list.append(10);
        linked_list.append(20);
        linked_list.append(30);
        linked_list.append(40);
        println!("{:?}", linked_list);
    }
    #[test]
    fn test_doubly_linked_list_pop_head() {
        let mut linked_list = DoublyLinkedList::new();
        linked_list.append(10);
        linked_list.append(20);
        linked_list.append(30);
        linked_list.append(40);
        linked_list.pop_head();
        println!("{:?}", linked_list);
    }
    #[test]
    fn test_doubly_linked_list_pop_tail() {
        let mut linked_list = DoublyLinkedList::new();
        linked_list.append(10);
        linked_list.append(20);
        linked_list.append(30);
        linked_list.append(40);
        linked_list.pop_tail();
        println!("{:?}", linked_list);
    }
    #[test]
    fn test_doubly_linked_list_remove_node() {
        let mut linked_list = DoublyLinkedList::new();
        linked_list.append(10);
        linked_list.append(20);
        let new_node = Rc::new(RefCell::new(Node {
            data: 30,
            prev: None,
            next: None,
        }));
        linked_list.append_node(new_node);
        linked_list.append(40);
        linked_list.pop_tail();
        println!("{:?}", linked_list);
    }
}
