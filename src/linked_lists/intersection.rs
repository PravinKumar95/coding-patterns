use crate::ds::linked_list::SinglyLinkedList;
use std::{ptr, rc::Rc};

fn run(list1: SinglyLinkedList, list2: SinglyLinkedList) -> bool {
    let mut ptr_1 = list1.peek().clone();
    let mut ptr_2 = list2.peek().clone();
    loop {
        match (&ptr_1, &ptr_2) {
            (Some(a), Some(b)) => {
                if Rc::ptr_eq(a, b) {
                    return true;
                }
            }
            (None, None) => break,
            _ => {}
        }
        ptr_1 = match ptr_1 {
            Some(node) => node.borrow().next.clone(),
            None => list2.peek().clone(),
        };
        ptr_2 = match ptr_2 {
            Some(node) => node.borrow().next.clone(),
            None => list1.peek().clone(),
        }
    }
    false
}

mod tests {
    use super::*;

    #[test]
    fn test_linked_list_intersection() {
        let mut list1 = SinglyLinkedList::new();
        let mut list2 = SinglyLinkedList::new();
        let mut shared = SinglyLinkedList::new();
        shared.append(8);
        shared.append(7);
        shared.append(2);

        list1.append(1);
        list1.append(3);
        list1.append(4);

        list2.append(6);
        list2.append(4);

        list1.append_node(shared.peek().unwrap());
        list2.append_node(shared.peek().unwrap());
        assert_eq!(run(list1, list2), true);
    }
}
