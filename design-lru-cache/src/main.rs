use std::{cell::RefCell, collections::HashMap, rc::Rc};
/// to implement a LRUCache we will use a double-ended LinkedList to add and remove the keys and a hashmap to store the node location
/// lets make a helper type for us to indicate a pointer to another node
type Link = Option<Rc<RefCell<Node>>>;
struct Node {
    key: i32,
    val: i32,
    prev: Link,
    next: Link,
}
impl Node {
    fn new(key: i32, val: i32) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(Node {
            key,
            val,
            prev: None,
            next: None,
        }))
    }
}
struct LRUCache {
    cap: usize,
    cache: HashMap<i32, Rc<RefCell<Node>>>,
    head: Rc<RefCell<Node>>,
    tail: Rc<RefCell<Node>>,
}
/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl LRUCache {
    fn new(capacity: i32) -> Self {
        let head = Node::new(-1, -1);
        let tail = Node::new(-1, -1);
        head.as_ref().borrow_mut().next = Some(Rc::clone(&tail));
        tail.as_ref().borrow_mut().prev = Some(Rc::clone(&head));
        Self {
            cap: capacity as usize,
            cache: HashMap::new(),
            head,
            tail,
        }
    }

    fn remove(&self, node: &Rc<RefCell<Node>>) {
        // we take the prev and next of this node and connect them
        let prev = node.as_ref().borrow().prev.clone().unwrap();
        let next = node.as_ref().borrow().next.clone().unwrap();
        prev.as_ref().borrow_mut().next = Some(Rc::clone(&next));
        next.as_ref().borrow_mut().prev = Some(Rc::clone(&prev));
    }

    fn insert(&self, node: &Rc<RefCell<Node>>) {
        let first = self.head.as_ref().borrow().next.clone().unwrap();
        node.as_ref().borrow_mut().next = Some(Rc::clone(&first));
        node.as_ref().borrow_mut().prev = Some(Rc::clone(&self.head));
        first.as_ref().borrow_mut().prev = Some(Rc::clone(node));
        self.head.as_ref().borrow_mut().next = Some(Rc::clone(node));
    }

    fn get(&self, key: i32) -> i32 {
        if let Some(node) = self.cache.get(&key) {
            let val = node.as_ref().borrow().val;
            self.remove(node);
            self.insert(node);
            val
        } else {
            -1
        }
    }

    fn put(&mut self, key: i32, value: i32) {
        if let Some(node) = self.cache.get(&key) {
            // Update the value
            node.as_ref().borrow_mut().val = value;
            self.remove(node);
            self.insert(node);
        } else {
            // Check if we need to evict
            if self.cache.len() == self.cap {
                let lru_key = self
                    .tail
                    .as_ref()
                    .borrow()
                    .prev
                    .as_ref()
                    .unwrap()
                    .as_ref()
                    .borrow()
                    .key;
                // Remove the least recently used node
                if let Some(lru_node) = self.cache.remove(&lru_key) {
                    self.remove(&lru_node);
                }
            }

            // Create and insert the new node
            let new_node = Node::new(key, value);
            self.insert(&new_node);
            self.cache.insert(key, new_node);
        }
    }
}
/**
 * Your LRUCache object will be instantiated and called as such:
 * let obj = LRUCache::new(capacity);
 * let ret_1: i32 = obj.get(key);
 * obj.put(key, value);
 */
// write the main function using ai
// write tests
fn main() {
    let mut lru = LRUCache::new(2);
    lru.put(1, 1);
    lru.put(2, 2);
    println!("{}", lru.get(1)); // 1
    lru.put(3, 3); // evicts key 2
    println!("{}", lru.get(2)); // -1
    lru.put(4, 4); // evicts key 1
    println!("{}", lru.get(1)); // -1
    println!("{}", lru.get(3)); // 3
    println!("{}", lru.get(4)); // 4
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_operations() {
        let mut lru = LRUCache::new(2);
        lru.put(1, 1);
        lru.put(2, 2);
        assert_eq!(lru.get(1), 1);
        lru.put(3, 3); // evicts key 2
        assert_eq!(lru.get(2), -1);
        lru.put(4, 4); // evicts key 1
        assert_eq!(lru.get(1), -1);
        assert_eq!(lru.get(3), 3);
        assert_eq!(lru.get(4), 4);
    }

    #[test]
    fn test_update_existing_key() {
        let mut lru = LRUCache::new(2);
        lru.put(1, 1);
        lru.put(2, 2);
        assert_eq!(lru.get(1), 1);
        lru.put(1, 10); // update value
        assert_eq!(lru.get(1), 10);
        lru.put(3, 3); // should evict key 2 since 1 was recently used
        assert_eq!(lru.get(2), -1);
        assert_eq!(lru.get(1), 10);
    }

    #[test]
    fn test_capacity_one() {
        let mut lru = LRUCache::new(1);
        lru.put(1, 1);
        assert_eq!(lru.get(1), 1);
        lru.put(2, 2);
        assert_eq!(lru.get(1), -1);
        assert_eq!(lru.get(2), 2);
    }
}
