use std::{
    cell::RefCell,
    collections::HashMap,
    rc::{self, Rc},
};

#[derive(Debug, Clone)]
struct Node {
    key: i32,
    val: i32,
    next: Option<Rc<RefCell<Node>>>,
    prev: Option<Rc<RefCell<Node>>>,
}

impl Node {
    fn new(key: i32, val: i32) -> Self {
        Self {
            key,
            val,
            next: None,
            prev: None,
        }
    }
}

#[derive(Debug)]
struct List {
    head: Rc<RefCell<Node>>,
    tail: Rc<RefCell<Node>>,
}

impl List {
    fn new() -> Self {
        let head = Rc::new(RefCell::new(Node::new(-1, -1)));
        let tail = Rc::new(RefCell::new(Node::new(-1, -1)));
        head.borrow_mut().next = Some(Rc::clone(&tail));
        tail.borrow_mut().prev = Some(Rc::clone(&head));
        Self { head, tail }
    }

    fn insert_from_head(&mut self, mut node: Node) -> Rc<RefCell<Node>> {
        let node_rc = Rc::new(RefCell::new(node));
        let next = Rc::clone(self.head.borrow().next.as_ref().unwrap());

        node_rc.borrow_mut().prev = Some(Rc::clone(&self.head));
        node_rc.borrow_mut().next = Some(Rc::clone(&next));

        self.head.borrow_mut().next = Some(Rc::clone(&node_rc));
        next.borrow_mut().prev = Some(Rc::clone(&node_rc));

        node_rc
    }

    fn remove_node(&mut self, key: i32) -> Option<Rc<RefCell<Node>>> {
        let mut current = Rc::clone(&self.head);

        loop {
            // Get next node if it exists (without holding the borrow)
            let next_opt = current.borrow().next.clone();

            match next_opt {
                Some(next_rc) => {
                    // Check if this is the node to remove
                    if next_rc.borrow().key == key {
                        // Get references to prev and next-next nodes
                        let prev_rc = next_rc.borrow().prev.clone().unwrap();
                        let next_next_opt = next_rc.borrow().next.clone();

                        // Update the links
                        prev_rc.borrow_mut().next = next_next_opt.clone();
                        if let Some(next_next_rc) = &next_next_opt {
                            next_next_rc.borrow_mut().prev = Some(prev_rc);
                        }

                        return Some(next_rc);
                    }

                    current = next_rc;
                }
                None => return None,
            }
        }
    }

    fn remove_tail(&mut self) -> Option<Rc<RefCell<Node>>> {
        let tail_prev = self.tail.borrow().prev.clone()?;
        let prev_prev = tail_prev.borrow().prev.clone()?;

        prev_prev.borrow_mut().next = Some(Rc::clone(&self.tail));
        self.tail.borrow_mut().prev = Some(Rc::clone(&prev_prev));

        Some(tail_prev)
    }
}

type Freq = usize;

struct LFUCache {
    capacity: i32,
    freq_map: HashMap<Freq, List>,
    cache: HashMap<i32, (i32, Freq, Rc<RefCell<Node>>)>,
    min_freq: Freq,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl LFUCache {
    fn new(capacity: i32) -> Self {
        Self {
            capacity,
            freq_map: HashMap::new(),
            cache: HashMap::new(),
            min_freq: 1,
        }
    }

    fn get(&mut self, key: i32) -> i32 {
        if self.capacity == 0 {
            return -1;
        }
        if let Some(&(val, freq, ref node_rc)) = self.cache.get(&key) {
            let new_freq = freq + 1;

            // Remove from old frequency list
            let mut old_list = self.freq_map.get_mut(&freq).unwrap();
            old_list.remove_node(key);
            if old_list.head.borrow().next.is_none() {
                self.freq_map.remove(&freq);
                if self.min_freq == freq {
                    self.min_freq = new_freq;
                }
            }
            // Insert into new frequency list
            let new_list = self.freq_map.entry(new_freq).or_insert_with(List::new);
            let node = new_list.insert_from_head(Node::new(key, val));

            // Update cache
            self.cache.insert(key, (val, new_freq, node));
            val
        } else {
            -1
        }
    }

    fn put(&mut self, key: i32, value: i32) {
        if self.capacity == 0 {
            return;
        }

        if let Some((_, freq, node_rc)) = self.cache.get_mut(&key) {
            // Key exists, update value and frequency
            let new_freq = *freq + 1;
            node_rc.borrow_mut().val = value; //update the value of the node.

            // Remove from old frequency list
            let mut old_list = self.freq_map.get_mut(freq).unwrap();
            old_list.remove_node(key);
            if old_list.head.borrow().next.is_none() {
                self.freq_map.remove(freq);
                if self.min_freq == *freq {
                    self.min_freq = new_freq;
                }
            }

            // Insert into new frequency list
            let new_list = self.freq_map.entry(new_freq).or_insert_with(List::new);
            let node = new_list.insert_from_head(Node::new(key, value));

            // Update cache
            self.cache.insert(key, (value, new_freq, node));
        } else {
            // Key does not exist, insert new node
            if self.cache.len() >= self.capacity as usize {
                // Cache is full, evict least frequently used item
                let min_list = self.freq_map.get_mut(&self.min_freq).unwrap();
                if let Some(evicted_node) = min_list.remove_tail() {
                    self.cache.remove(&evicted_node.borrow().key);
                }
                if min_list.head.borrow().next.is_none() {
                    self.freq_map.remove(&self.min_freq);
                }
            }

            // Insert new node into frequency 1 list
            let new_list = self.freq_map.entry(1).or_insert_with(List::new);
            let node_rc = new_list.insert_from_head(Node::new(key, value));
            self.cache.insert(key, (value, 1, node_rc));
            self.min_freq = 1;
        }
    }
}

/**
 * Your LFUCache object will be instantiated and called as such:
 * let obj = LFUCache::new(capacity);
 * let ret_1: i32 = obj.get(key);
 * obj.put(key, value);
 */

fn main() {
    let mut lfu_cache = LFUCache::new(2);
    lfu_cache.put(1, 1);
    lfu_cache.put(2, 2);
    println!("get(1): {}", lfu_cache.get(1)); // returns 1
    lfu_cache.put(3, 3); // evicts key 2
    println!("get(2): {}", lfu_cache.get(2)); // returns -1 (not found)
    println!("get(3): {}", lfu_cache.get(3)); // returns 3
    lfu_cache.put(4, 4); // evicts key 1
    println!("get(1): {}", lfu_cache.get(1)); // returns -1 (not found)
    println!("get(3): {}", lfu_cache.get(3)); // returns 3
    println!("get(4): {}", lfu_cache.get(4)); // returns 4

    let mut lfu_cache2 = LFUCache::new(3);
    lfu_cache2.put(1, 1);
    lfu_cache2.put(2, 2);
    lfu_cache2.put(3, 3);
    lfu_cache2.get(1);
    lfu_cache2.get(2);
    lfu_cache2.get(3);
    lfu_cache2.put(4, 4);
    println!("Cache 2: get(1): {}", lfu_cache2.get(1));
    println!("Cache 2: get(2): {}", lfu_cache2.get(2));
    println!("Cache 2: get(3): {}", lfu_cache2.get(3));
    println!("Cache 2: get(4): {}", lfu_cache2.get(4));
}
