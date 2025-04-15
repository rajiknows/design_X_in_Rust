struct PeekingIterator<I: Iterator> {
    iterator: I,
    peeked: Option<I::Item>,
}

impl<I: Iterator> PeekingIterator<I> {
    fn new(iter: I) -> Self {
        PeekingIterator {
            iterator: iter,
            peeked: None,
        }
    }

    fn peek(&mut self) -> Option<&I::Item> {
        // If we haven't peeked yet, get the next item and store it
        if self.peeked.is_none() {
            self.peeked = self.iterator.next();
        }

        // Return a reference to the peeked value (if any)
        self.peeked.as_ref()
    }

    fn next(&mut self) -> Option<I::Item> {
        // If we have a peeked value, return it
        if self.peeked.is_some() {
            // Take ownership of the peeked value, leaving None in its place
            return self.peeked.take();
        }

        // Otherwise, get the next value from the iterator
        self.iterator.next()
    }

    fn has_next(&mut self) -> bool {
        // Peek to load the next value if needed, then check if it exists
        self.peek().is_some()
    }
}

fn main() {
    // Example 1 test
    test_example_1();

    // Additional tests
    test_empty_iterator();
    test_single_element();
    test_alternating_peek_next();
    test_multiple_peeks();

    println!("All tests passed!");
}

fn test_example_1() {
    // Recreate the example from problem statement
    let vec = vec![1, 2, 3];
    let mut peekingIterator = PeekingIterator::new(vec.into_iter());

    // Test operations
    assert_eq!(
        peekingIterator.next(),
        Some(1),
        "First next() should return 1"
    );
    assert_eq!(
        peekingIterator.peek(),
        Some(&2),
        "peek() should return reference to 2"
    );
    assert_eq!(
        peekingIterator.next(),
        Some(2),
        "Second next() should return 2"
    );
    assert_eq!(
        peekingIterator.next(),
        Some(3),
        "Third next() should return 3"
    );
    assert_eq!(
        peekingIterator.has_next(),
        false,
        "hasNext() should return false after all elements consumed"
    );

    println!("Example 1 test passed!");
}

fn test_empty_iterator() {
    // Test with an empty iterator
    let vec: Vec<i32> = vec![];
    let mut peekingIterator = PeekingIterator::new(vec.into_iter());

    assert_eq!(
        peekingIterator.has_next(),
        false,
        "hasNext() should return false for empty iterator"
    );
    assert_eq!(
        peekingIterator.peek(),
        None,
        "peek() should return None for empty iterator"
    );
    assert_eq!(
        peekingIterator.next(),
        None,
        "next() should return None for empty iterator"
    );

    println!("Empty iterator test passed!");
}

fn test_single_element() {
    // Test with a single element
    let vec = vec![42];
    let mut peekingIterator = PeekingIterator::new(vec.into_iter());

    assert_eq!(
        peekingIterator.has_next(),
        true,
        "hasNext() should return true"
    );
    assert_eq!(
        peekingIterator.peek(),
        Some(&42),
        "peek() should return reference to 42"
    );
    assert_eq!(
        peekingIterator.has_next(),
        true,
        "hasNext() should still return true after peek()"
    );
    assert_eq!(peekingIterator.next(), Some(42), "next() should return 42");
    assert_eq!(
        peekingIterator.has_next(),
        false,
        "hasNext() should return false after element consumed"
    );
    assert_eq!(
        peekingIterator.peek(),
        None,
        "peek() should return None after all elements consumed"
    );
    assert_eq!(
        peekingIterator.next(),
        None,
        "next() should return None after all elements consumed"
    );

    println!("Single element test passed!");
}

fn test_alternating_peek_next() {
    // Test alternating between peek and next
    let vec = vec![1, 2, 3, 4, 5];
    let mut peekingIterator = PeekingIterator::new(vec.into_iter());

    assert_eq!(
        peekingIterator.peek(),
        Some(&1),
        "First peek() should return reference to 1"
    );
    assert_eq!(
        peekingIterator.next(),
        Some(1),
        "First next() should return 1"
    );
    assert_eq!(
        peekingIterator.peek(),
        Some(&2),
        "Second peek() should return reference to 2"
    );
    assert_eq!(
        peekingIterator.next(),
        Some(2),
        "Second next() should return 2"
    );
    assert_eq!(
        peekingIterator.next(),
        Some(3),
        "Third next() should return 3 without peeking"
    );
    assert_eq!(
        peekingIterator.peek(),
        Some(&4),
        "Third peek() should return reference to 4"
    );
    assert_eq!(
        peekingIterator.peek(),
        Some(&4),
        "Repeated peek() should still return reference to 4"
    );
    assert_eq!(
        peekingIterator.next(),
        Some(4),
        "Fourth next() should return 4"
    );
    assert_eq!(
        peekingIterator.next(),
        Some(5),
        "Fifth next() should return 5"
    );
    assert_eq!(
        peekingIterator.has_next(),
        false,
        "hasNext() should return false after all elements consumed"
    );

    println!("Alternating peek/next test passed!");
}

fn test_multiple_peeks() {
    // Test multiple peeks before next
    let vec = vec![10, 20, 30];
    let mut peekingIterator = PeekingIterator::new(vec.into_iter());

    assert_eq!(
        peekingIterator.peek(),
        Some(&10),
        "First peek() should return reference to 10"
    );
    assert_eq!(
        peekingIterator.peek(),
        Some(&10),
        "Second peek() should still return reference to 10"
    );
    assert_eq!(
        peekingIterator.peek(),
        Some(&10),
        "Third peek() should still return reference to 10"
    );
    assert_eq!(
        peekingIterator.next(),
        Some(10),
        "next() should return 10 after multiple peeks"
    );
    assert_eq!(
        peekingIterator.peek(),
        Some(&20),
        "peek() after next() should return reference to 20"
    );
    assert_eq!(
        peekingIterator.peek(),
        Some(&20),
        "Multiple peeks should continue to return reference to 20"
    );
    assert_eq!(peekingIterator.next(), Some(20), "next() should return 20");
    assert_eq!(
        peekingIterator.next(),
        Some(30),
        "next() without peek should return 30"
    );
    assert_eq!(
        peekingIterator.has_next(),
        false,
        "hasNext() should return false after all elements consumed"
    );

    println!("Multiple peeks test passed!");
}
