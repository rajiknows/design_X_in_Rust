// This is a sample Rust program that implements a Trie data structure.
// A Trie is a tree-like data structure that stores a dynamic set of strings,
// where the keys are usually strings.
// It is commonly used for autocomplete and spell checking.

#[derive(Debug)]
struct TrieNode {
    children: [Option<Box<TrieNode>>; 26],
    is_end: bool,
}

impl TrieNode {
    fn new() -> Self {
        Self {
            // Create an array of None values for children
            children: Default::default(),
            is_end: false,
        }
    }
}

struct Trie {
    root: TrieNode,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl Trie {
    fn new() -> Self {
        Self {
            root: TrieNode::new(),
        }
    }

    fn insert(&mut self, word: String) {
        let mut current = &mut self.root;
        for ch in word.chars() {
            let index = (ch as u8 - b'a') as usize;
            if current.children[index].is_none() {
                current.children[index] = Some(Box::new(TrieNode::new()));
            }
            // Move to the next node
            current = current.children[index].as_mut().unwrap();
        }
        current.is_end = true;
    }

    fn search(&self, word: String) -> bool {
        let mut current = &self.root;
        for ch in word.chars() {
            let index = (ch as u8 - b'a') as usize;
            match &current.children[index] {
                None => return false,
                Some(node) => current = node,
            }
        }
        current.is_end
    }

    fn starts_with(&self, prefix: String) -> bool {
        let mut current = &self.root;
        for ch in prefix.chars() {
            let index = (ch as u8 - b'a') as usize;
            match &current.children[index] {
                None => return false,
                Some(node) => current = node,
            }
        }
        true
    }
}

/**
 * Your Trie object will be instantiated and called as such:
 * let obj = Trie::new();
 * obj.insert(word);
 * let ret_2: bool = obj.search(word);
 * let ret_3: bool = obj.starts_with(prefix);
 */
fn main() {
    let mut trie = Trie::new();

    // Insert some words
    trie.insert("apple".to_string());
    trie.insert("application".to_string());
    trie.insert("banana".to_string());

    // Test search
    println!("Search 'apple': {}", trie.search("apple".to_string())); // true
    println!("Search 'app': {}", trie.search("app".to_string())); // false
    println!("Search 'banana': {}", trie.search("banana".to_string())); // true
    println!("Search 'ban': {}", trie.search("ban".to_string())); // false

    // Test starts_with
    println!("Starts with 'app': {}", trie.starts_with("app".to_string())); // true
    println!("Starts with 'ban': {}", trie.starts_with("ban".to_string())); // true
    println!("Starts with 'bat': {}", trie.starts_with("bat".to_string())); // false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_trie_operations() {
        let mut trie = Trie::new();

        // Insert words
        trie.insert("apple".to_string());
        trie.insert("application".to_string());

        // Test search
        assert!(trie.search("apple".to_string()));
        assert!(!trie.search("app".to_string()));
        assert!(trie.search("application".to_string()));
        assert!(!trie.search("appl".to_string()));

        // Test starts_with
        assert!(trie.starts_with("app".to_string()));
        assert!(trie.starts_with("appl".to_string()));
        assert!(trie.starts_with("apple".to_string()));
        assert!(!trie.starts_with("banana".to_string()));

        // Add more words
        trie.insert("banana".to_string());
        assert!(trie.search("banana".to_string()));
        assert!(trie.starts_with("ban".to_string()));
    }

    #[test]
    fn test_empty_trie() {
        let trie = Trie::new();
        assert!(!trie.search("anything".to_string()));
        assert!(!trie.starts_with("anything".to_string()));
    }
}
