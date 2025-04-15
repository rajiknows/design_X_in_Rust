const std = @import("std");
const testing = std.testing;
const Allocator = std.mem.Allocator;

/// TrieNode represents a single node in the Trie data structure
const TrieNode = struct {
    /// Children nodes, one for each lowercase letter (a-z)
    children: [26]?*TrieNode,
    /// Flag indicating if this node marks the end of a word
    is_end: bool,

    /// Create a new TrieNode with all children initialized to null
    fn init(allocator: Allocator) !*TrieNode {
        var node = try allocator.create(TrieNode);
        // Initialize all children to null
        for (&node.children) |*child| {
            child.* = null;
        }
        node.is_end = false;
        return node;
    }

    /// Free the memory of this node and all its children recursively
    fn deinit(self: *TrieNode, allocator: Allocator) void {
        // First free all non-null children recursively
        for (self.children) |child_opt| {
            if (child_opt) |child| {
                child.deinit(allocator);
            }
        }
        // Then free this node
        allocator.destroy(self);
    }
};

/// Trie is a tree-like data structure for efficient string storage and retrieval
pub const Trie = struct {
    root: *TrieNode,
    allocator: Allocator,

    /// Create a new Trie
    pub fn init(allocator: Allocator) !Trie {
        const root = try TrieNode.init(allocator);
        return Trie{
            .root = root,
            .allocator = allocator,
        };
    }

    /// Clean up all allocated memory
    pub fn deinit(self: *Trie) void {
        self.root.deinit(self.allocator);
    }

    /// Insert a word into the Trie
    pub fn insert(self: *Trie, word: []const u8) !void {
        var current = self.root;

        for (word) |char| {
            // Ensure the character is a lowercase letter
            if (char < 'a' or char > 'z') {
                return error.InvalidCharacter;
            }

            const index = char - 'a';

            // Create new node if needed
            if (current.children[index] == null) {
                current.children[index] = try TrieNode.init(self.allocator);
            }

            // Move to next node
            current = current.children[index].?;
        }

        // Mark end of word
        current.is_end = true;
    }

    /// Search for a complete word in the Trie
    pub fn search(self: *const Trie, word: []const u8) bool {
        var current = self.root;

        for (word) |char| {
            if (char < 'a' or char > 'z') {
                return false;
            }

            const index = char - 'a';

            // If child doesn't exist, word is not in trie
            if (current.children[index] == null) {
                return false;
            }

            current = current.children[index].?;
        }

        // Word exists only if we're at an end node
        return current.is_end;
    }

    /// Check if any word in the Trie starts with the given prefix
    pub fn startsWith(self: *const Trie, prefix: []const u8) bool {
        var current = self.root;

        for (prefix) |char| {
            if (char < 'a' or char > 'z') {
                return false;
            }

            const index = char - 'a';

            // If child doesn't exist, prefix is not in trie
            if (current.children[index] == null) {
                return false;
            }

            current = current.children[index].?;
        }

        // We found the prefix
        return true;
    }
};

// Test cases
test "Trie basic operations" {
    var arena = std.heap.ArenaAllocator.init(std.testing.allocator);
    defer arena.deinit();
    const allocator = arena.allocator();

    var trie = try Trie.init(allocator);
    defer trie.deinit();

    // Insert words
    try trie.insert("apple");
    try trie.insert("application");
    try trie.insert("banana");

    // Search for complete words
    try testing.expect(trie.search("apple"));
    try testing.expect(!trie.search("app"));
    try testing.expect(trie.search("application"));
    try testing.expect(trie.search("banana"));
    try testing.expect(!trie.search("ban"));

    // Check prefixes
    try testing.expect(trie.startsWith("app"));
    try testing.expect(trie.startsWith("ban"));
    try testing.expect(!trie.startsWith("orange"));
}

// Example main function to demonstrate usage
pub fn main() !void {
    // Create a general purpose allocator
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    defer _ = gpa.deinit();
    const allocator = gpa.allocator();

    // Create a new Trie
    var trie = try Trie.init(allocator);
    defer trie.deinit();

    // Insert some words
    try trie.insert("hello");
    try trie.insert("world");
    try trie.insert("help");
    try trie.insert("welcome");

    // Print to standard output
    const stdout = std.io.getStdOut().writer();

    try stdout.print("Search 'hello': {}\n", .{trie.search("hello")});
    try stdout.print("Search 'help': {}\n", .{trie.search("help")});
    try stdout.print("Search 'hell': {}\n", .{trie.search("hell")});
    try stdout.print("Starts with 'hel': {}\n", .{trie.startsWith("hel")});
    try stdout.print("Starts with 'wor': {}\n", .{trie.startsWith("wor")});
    try stdout.print("Starts with 'abc': {}\n", .{trie.startsWith("abc")});
}
