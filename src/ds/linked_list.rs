use std::cell::RefCell;
use std::rc::Rc;

type Link = Option<Rc<RefCell<SNode>>>;

#[derive(Debug)]
struct SNode {
    data: i32,
    next: Link,
}

#[derive(Debug)]
struct SinglyLinkedList {
    head: Link,
}

impl SinglyLinkedList {
    fn new() -> Self {
        SinglyLinkedList { head: None }
    }
    fn append(&mut self, val: i32) {
        let new_node = Rc::new(RefCell::new(SNode {
            data: val,
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
    fn insert(&mut self, val: i32, index: usize) {
        let new_node = Rc::new(RefCell::new(SNode {
            data: val,
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
    fn pop(&mut self) -> i32 {
        if let Some(node) = self.head.as_mut() {
            let mut curr = node.clone();
            let mut prev = curr.clone();
            while !curr.borrow().next.is_none() {
                prev = curr.clone();
                let next = curr.borrow().next.clone();
                curr = next.unwrap();
            }
            let out = prev.borrow_mut().next.take();
            return out.unwrap().borrow().data;
        }
        0
    }
    fn remove_kth_last(&mut self, index: usize) -> i32 {
        if index == 0 {
            return 0;
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
                    return 0;
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
            return out.unwrap().borrow().data;
        }
        0
    }
}

struct DoublyLinkedList {}

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
}
