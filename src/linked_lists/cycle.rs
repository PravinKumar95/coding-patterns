use crate::ds::linked_list::SinglyLinkedList;
use std::{cell::RefCell, rc::Rc};

use crate::ds::linked_list::Node;

fn run(list: SinglyLinkedList<i32>) -> bool {
    let mut slow = list.peek();
    let mut fast = list.peek();
    loop {
        if fast.is_none() {
            return false;
        }
        let fast_next = fast.unwrap().borrow().next.clone();
        if fast_next.is_none() {
            return false;
        }
        let fast_next_next = fast_next.unwrap().borrow().next.clone();
        fast = fast_next_next;
        slow = slow.unwrap().borrow().next.clone();
        match (slow.clone(), fast.clone()) {
            (Some(slow_ptr), Some(fast_ptr)) => {
                if Rc::ptr_eq(&slow_ptr, &fast_ptr) {
                    return true;
                }
            }
            _ => {}
        }
    }
    false
}

mod tests {
    use super::*;
    #[test]
    fn test_cycle_true() {
        let mut list = SinglyLinkedList::new();
        let node_two = Rc::new(RefCell::new(Node {
            data: 2,
            prev: None,
            next: None,
        }));
        let node_five = Rc::new(RefCell::new(Node {
            data: 5,
            prev: None,
            next: Some(node_two.clone()),
        }));
        list.append(0);
        list.append(1);
        list.append_node(node_two);
        list.append(3);
        list.append(4);
        list.append_node(node_five);
        assert_eq!(run(list), true);
    }
    #[test]
    fn test_cycle_false() {
        let mut list = SinglyLinkedList::new();
        list.append(0);
        list.append(1);
        list.append(2);
        list.append(3);
        list.append(4);
        list.append(5);
        assert_eq!(run(list), false);
    }
}
