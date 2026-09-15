#[derive(Debug)]
struct SNode {
    data: i32,
    next: Option<Box<SNode>>,
}

#[derive(Debug)]
struct SinglyLinkedList {
    head: Option<Box<SNode>>,
    count: usize,
}

impl SinglyLinkedList {
    fn new() -> Self {
        SinglyLinkedList {
            head: None,
            count: 0,
        }
    }
    fn append(&mut self, val: i32) {
        let mut curr = &mut self.head;
        if curr.is_none() {
            *curr = Some(Box::new(SNode {
                data: val,
                next: None,
            }));
            self.count += 1;
            return;
        }
        while !curr.is_none() && !curr.as_ref().unwrap().next.is_none() {
            curr = &mut curr.as_deref_mut().unwrap().next;
        }
        println!("curr: {:?}", &curr);
        curr.as_deref_mut().unwrap().next = Some(Box::new(SNode {
            data: val,
            next: None,
        }));
        self.count += 1;
    }
    fn insert(&mut self, val: i32, index: usize) {
        let mut head = &mut self.head.take();
        if head.is_none() {
            head.replace(Box::new(SNode {
                data: val,
                next: None,
            }));
            self.count += 1;
            self.head = head.take();
            return;
        }
        let mut new_node = Some(Box::new(SNode {
            data: val,
            next: None,
        }));
        if index == 0 {
            new_node
                .as_deref_mut()
                .unwrap()
                .next
                .replace(head.take().unwrap());
            head.replace(new_node.unwrap());
            self.head = head.take();
            return;
        }
        self.head = head.take();
        let mut curr = &mut self.head;
        let mut curr_idx = 1;
        while !curr.is_none() && curr_idx < index {
            curr = &mut curr.as_deref_mut().unwrap().next;
            curr_idx += 1;
        }
        println!("curr: {:?}", &curr);
        if curr.is_none() {
            let _ = curr.insert(new_node.unwrap());
            self.count += 1;
            return;
        }
        let next = curr.as_deref_mut().unwrap().next.take();
        curr.as_deref_mut().unwrap().next = new_node;
        curr.as_deref_mut()
            .unwrap()
            .next
            .as_deref_mut()
            .unwrap()
            .next = next;
        self.count += 1;
    }
    fn reverse(&mut self) {
        let mut head = self.head.take();
        let mut prev = None;
        let mut curr = head;
        while let Some(mut node) = curr {
            let next = node.next.take();
            node.next = prev.take();
            prev = Some(node);
            curr = next;
        }
        self.head = prev;
    }
    fn pop(&mut self) -> i32 {
        let mut curr = &mut self.head;
        if curr.is_none() {
            return 0;
        }
        while !curr.is_none() && !curr.as_ref().unwrap().next.is_none() {
            curr = &mut curr.as_deref_mut().unwrap().next;
        }
        let out = curr.take();
        out.unwrap().data
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
}
