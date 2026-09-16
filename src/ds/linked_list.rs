use std::cell::RefCell;
use std::rc::Rc;

type Link = Option<Rc<RefCell<SNode>>>;

#[derive(Debug)]
struct SNode {
    data: i32,
    next: Option<usize>,
}

#[derive(Debug)]
struct SinglyLinkedList {
    nodes: Vec<SNode>,
}

impl SinglyLinkedList {
    fn new() -> Self {
        SinglyLinkedList { nodes: vec![] }
    }
    fn append(&mut self, val: i32) {
        let new_node = SNode {
            data: val,
            next: None,
        };
        if self.nodes.is_empty() {
            self.nodes.push(new_node);
            return;
        }
        self.nodes.push(new_node);
    }
    fn insert(&mut self, val: i32, index: usize) {
        let new_node = SNode {
            data: val,
            next: None,
        };
        if self.nodes.is_empty() {
            self.nodes.push(new_node);
            return;
        }
        if index >= self.nodes.len() {
            self.nodes.push(new_node);
            return;
        }
        self.nodes.insert(index, new_node);
    }
    fn reverse(&mut self) {
        self.nodes.reverse();
    }
    fn pop(&mut self) -> i32 {
        self.nodes.pop().unwrap().data
    }
    fn remove_kth_last(&mut self, index: usize) -> i32 {
        let node = self.nodes.remove(self.nodes.len() - 1 - index);
        return node.data;
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
    fn test_singly_linked_list_insert() {
        let mut linked_list = SinglyLinkedList::new();
        linked_list.insert(10, 10);
        linked_list.append(30);
        linked_list.append(40);
        linked_list.insert(20, 5);
        println!("{:?}", linked_list);
    }
    #[test]
    fn test_singly_linked_list_pop() {
        let mut linked_list = SinglyLinkedList::new();
        linked_list.insert(10, 10);
        assert_eq!(linked_list.pop(), 10);
    }
    #[test]
    fn test_singly_linked_list_reverse() {
        let mut linked_list = SinglyLinkedList::new();
        linked_list.insert(10, 10);
        linked_list.append(30);
        linked_list.append(40);
        linked_list.insert(20, 1);
        linked_list.reverse();
        println!("{:?}", linked_list);
    }
    #[test]
    fn test_singly_linked_list_remove_kth_last() {
        let mut linked_list = SinglyLinkedList::new();
        linked_list.insert(10, 10);
        linked_list.append(30);
        linked_list.append(40);
        linked_list.insert(20, 1);
        linked_list.remove_kth_last(1);
        println!("{:?}", linked_list);
    }
}
