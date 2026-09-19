use crate::ds::linked_list::SinglyLinkedList;
use std::{cell::RefCell, rc::Rc};

use crate::ds::linked_list::Node;

fn run(list: SinglyLinkedList<i32>) -> i32 {
    let mut slow = list.peek();
    let mut fast = list.peek();
    loop {
        if fast.is_none() {
            return slow.unwrap().borrow().data;
        }
        let fast_next = fast.unwrap().borrow().next.clone();
        if fast_next.is_none() {
            return slow.unwrap().borrow().data;
        }
        let fast_next_next = fast_next.unwrap().borrow().next.clone();
        fast = fast_next_next;
        slow = slow.unwrap().borrow().next.clone();
    }
}

mod tests {
    use super::*;
    #[test]
    fn test_odd_midpoint() {
        let mut list = SinglyLinkedList::new();
        list.append(0);
        list.append(1);
        list.append(2);
        list.append(3);
        list.append(4);
        assert_eq!(run(list), 2);
    }
    #[test]
    fn test_even_midpoint() {
        let mut list = SinglyLinkedList::new();
        list.append(0);
        list.append(1);
        list.append(2);
        list.append(3);
        assert_eq!(run(list), 2);
    }
}
