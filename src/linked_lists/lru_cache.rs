use std::collections::HashMap;

use crate::ds::linked_list::{DoublyLinkedList, Link};

type Data = (i32, i32);

#[derive(Debug)]
struct LRUCache {
    linked_list: DoublyLinkedList<Data>,
    hash_map: HashMap<i32, Link<Data>>,
    capacity: usize,
}
impl LRUCache {
    pub fn new(capacity: usize) -> Self {
        Self {
            linked_list: DoublyLinkedList::new(),
            hash_map: HashMap::new(),
            capacity,
        }
    }
    pub fn put(&mut self, key: i32, val: i32) {
        if self.hash_map.len() >= self.capacity {
            let el = self.linked_list.pop_head().unwrap();
            self.hash_map.remove(&el.borrow().data.0);
        }
        self.linked_list.append((key, val));
        self.hash_map.insert(key, self.linked_list.peek_tail());
    }
    pub fn get(&mut self, key: i32) -> i32 {
        let val = self.hash_map.get(&key);
        if val.is_none() {
            return 0;
        }
        let node = val.unwrap().as_ref().unwrap();
        println!("Curr Node: {:?}", &node);
        self.linked_list.remove_node(node.clone());
        println!("Linked list after remove node: {:?}", self.linked_list);
        self.linked_list.append_node(node.clone());
        println!("Linked list after append node: {:?}", self.linked_list);
        return 0;
    }
}
mod test {
    use super::*;
    #[test]
    fn test_lru_cache() {
        let mut cache = LRUCache::new(3);
        cache.put(1, 100);
        cache.put(2, 200);
        cache.put(3, 300);
        cache.put(4, 400);
        cache.get(2);
        println!("{:?}", &cache.linked_list);
    }
}
