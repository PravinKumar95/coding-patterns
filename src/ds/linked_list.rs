struct SNode {
    data: i32,
    next: Option<i32> 
}

struct SinglyLinkedList {
    head: Option<SNode>,
}

impl SinglyLinkedList {
    fn new() -> Self {
        SinglyLinkedList { head: None }
    }
}

struct DoublyLinkedList {

}

use std::collections::linked_list;

