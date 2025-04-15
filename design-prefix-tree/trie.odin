package main

import "core:fmt"
import "core:mem"
import "core:testing"

// TrieNode represents a single node in our Trie structure
TrieNode :: struct {
    // Array of pointers to child nodes, one for each lowercase letter (a-z)
    children: [26]^TrieNode,
    // Flag indicating if this node marks the end of a word
    is_end: bool,
}

// Create a new TrieNode with all children initialized to nil
new_trie_node :: proc(allocator: mem.Allocator) -> ^TrieNode {
    node := new(TrieNode, allocator)

    // Initialize all children to nil
    for i in 0..<26 {
        node.children[i] = nil
    }

    node.is_end = false
    return node
}

// Delete a TrieNode and all its children recursively
delete_trie_node :: proc(node: ^TrieNode, allocator: mem.Allocator) {
    if node == nil {
        return
    }

    // First delete all non-nil children recursively
    for i in 0..<26 {
        if node.children[i] != nil {
            delete_trie_node(node.children[i], allocator)
        }
    }

    // Then delete this node
    free(node, allocator)
}

// Trie is a tree-like data structure for efficient string operations
Trie :: struct {
    root: ^TrieNode,
    allocator: mem.Allocator,
}

// Initialize a new Trie
new_trie :: proc(allocator: mem.Allocator) -> Trie {
    return Trie{
        root = new_trie_node(allocator),
        allocator = allocator,
    }
}

// Clean up all allocated memory
delete_trie :: proc(trie: ^Trie) {
    delete_trie_node(trie.root, trie.allocator)
    trie.root = nil
}

// Insert a word into the Trie
trie_insert :: proc(trie: ^Trie, word: string) {
    if trie.root == nil {
        return
    }

    current := trie.root

    for c in word {
        // Ensure the character is a lowercase letter
        if c < 'a' || c > 'z' {
            fmt.println("Warning: Skipping invalid character:", c)
            continue
        }

        index := int(c - 'a')

        // Create new node if needed
        if current.children[index] == nil {
            current.children[index] = new_trie_node(trie.allocator)
        }

        // Move to next node
        current = current.children[index]
    }

    // Mark the end of the word
    current.is_end = true
}

// Search for a complete word in the Trie
trie_search :: proc(trie: ^Trie, word: string) -> bool {
    if trie.root == nil {
        return false
    }

    current := trie.root

    for c in word {
        if c < 'a' || c > 'z' {
            return false
        }

        index := int(c - 'a')

        // If child doesn't exist, word is not in trie
        if current.children[index] == nil {
            return false
        }

        current = current.children[index]
    }

    // Word exists only if we're at an end node
    return current.is_end
}

// Check if any word in the Trie starts with the given prefix
trie_starts_with :: proc(trie: ^Trie, prefix: string) -> bool {
    if trie.root == nil {
        return false
    }

    current := trie.root

    for c in prefix {
        if c < 'a' || c > 'z' {
            return false
        }

        index := int(c - 'a')

        // If child doesn't exist, prefix is not in trie
        if current.children[index] == nil {
            return false
        }

        current = current.children[index]
    }

    // We found the prefix
    return true
}

// Test functions for our Trie implementation
@(test)
test_trie_basic_operations :: proc(t: ^testing.T) {
    // Create a trie using the default allocator
    trie := new_trie(context.allocator)
    defer delete_trie(&trie)

    // Insert words
    trie_insert(&trie, "apple")
    trie_insert(&trie, "application")
    trie_insert(&trie, "banana")

    // Search for complete words
    testing.expect(t, trie_search(&trie, "apple"), "Expected 'apple' to be found")
    testing.expect(t, !trie_search(&trie, "app"), "Expected 'app' to not be found")
    testing.expect(t, trie_search(&trie, "application"), "Expected 'application' to be found")
    testing.expect(t, trie_search(&trie, "banana"), "Expected 'banana' to be found")
    testing.expect(t, !trie_search(&trie, "ban"), "Expected 'ban' to not be found")

    // Check prefixes
    testing.expect(t, trie_starts_with(&trie, "app"), "Expected 'app' prefix to be found")
    testing.expect(t, trie_starts_with(&trie, "ban"), "Expected 'ban' prefix to be found")
    testing.expect(t, !trie_starts_with(&trie, "orange"), "Expected 'orange' prefix to not be found")
}

main :: proc() {
    // Create a trie using the default allocator
    trie := new_trie(context.allocator)
    defer delete_trie(&trie)

    // Insert some words
    trie_insert(&trie, "hello")
    trie_insert(&trie, "world")
    trie_insert(&trie, "help")
    trie_insert(&trie, "welcome")

    // Print results
    fmt.println("Search 'hello':", trie_search(&trie, "hello"))
    fmt.println("Search 'help':", trie_search(&trie, "help"))
    fmt.println("Search 'hell':", trie_search(&trie, "hell"))
    fmt.println("Starts with 'hel':", trie_starts_with(&trie, "hel"))
    fmt.println("Starts with 'wor':", trie_starts_with(&trie, "wor"))
    fmt.println("Starts with 'abc':", trie_starts_with(&trie, "abc"))
}
