/// Rust does not have a built-in stack data structure, but we can implement one using a vector.
struct Stack<T> {
    stack: Vec<T>,
}
impl<T> Stack<T> {
    fn new() -> Self {
        Stack { stack: Vec::new() }
    }
    fn push(&mut self, value: T) {
        self.stack.push(value);
    }
    fn pop(&mut self) -> Option<T> {
        self.stack.pop()
    }
    fn top(&self) -> Option<&T> {
        self.stack.last()
    }
    fn is_empty(&self) -> bool {
        self.stack.is_empty()
    }
}

// We'll remove the get_min method from Stack as it only works for i32
// and instead implement it inside the MinStack

struct MinStack {
    stack: Stack<(i32, i32)>, // (value, current_min)
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MinStack {
    fn new() -> Self {
        MinStack {
            stack: Stack::new(),
        }
    }

    fn push(&mut self, val: i32) {
        let min = if self.stack.is_empty() {
            val
        } else {
            std::cmp::min(val, self.get_min())
        };
        self.stack.push((val, min));
    }

    fn pop(&mut self) {
        self.stack.pop();
    }

    fn top(&self) -> i32 {
        match self.stack.top() {
            Some((val, _)) => *val,
            None => -1,
        }
    }

    fn get_min(&self) -> i32 {
        match self.stack.top() {
            Some((_, min)) => *min,
            None => -1,
        }
    }
}

/**
 * Your MinStack object will be instantiated and called as such:
 * let obj = MinStack::new();
 * obj.push(val);
 * obj.pop();
 * let ret_3: i32 = obj.top();
 * let ret_4: i32 = obj.get_min();
 */
fn main() {
    println!("Hello, world!");
    let mut min_stack = MinStack::new();
    min_stack.push(3);
    min_stack.push(5);
    min_stack.push(2);
    min_stack.push(1);
    min_stack.push(4);
    println!("Top: {}", min_stack.top()); // 4
    println!("Min: {}", min_stack.get_min()); // 1
    min_stack.pop();
    println!("Top: {}", min_stack.top()); // 1
    println!("Min: {}", min_stack.get_min()); // 1
    min_stack.pop();
    println!("Top: {}", min_stack.top()); // 2
    println!("Min: {}", min_stack.get_min()); // 2
    min_stack.pop();
    println!("Top: {}", min_stack.top()); // 5
    println!("Min: {}", min_stack.get_min()); // 3
    min_stack.pop();
    println!("Top: {}", min_stack.top()); // 3
    println!("Min: {}", min_stack.get_min()); // 3
    min_stack.pop();
    println!("Top: {}", min_stack.top()); // -1
    println!("Min: {}", min_stack.get_min()); // -1
    min_stack.pop();
    println!("Top: {}", min_stack.top()); // -1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_min_stack() {
        let mut min_stack = MinStack::new();
        min_stack.push(3);
        min_stack.push(5);
        min_stack.push(2);
        min_stack.push(1);
        min_stack.push(4);

        assert_eq!(min_stack.top(), 4);
        assert_eq!(min_stack.get_min(), 1);

        min_stack.pop();
        assert_eq!(min_stack.top(), 1);
        assert_eq!(min_stack.get_min(), 1);

        min_stack.pop();
        assert_eq!(min_stack.top(), 2);
        assert_eq!(min_stack.get_min(), 2);

        min_stack.pop();
        assert_eq!(min_stack.top(), 5);
        assert_eq!(min_stack.get_min(), 3);

        min_stack.pop();
        assert_eq!(min_stack.top(), 3);
        assert_eq!(min_stack.get_min(), 3);

        min_stack.pop();
        assert_eq!(min_stack.top(), -1);
        assert_eq!(min_stack.get_min(), -1);
    }

    #[test]
    fn test_empty_stack() {
        let min_stack = MinStack::new();
        assert_eq!(min_stack.top(), -1);
        assert_eq!(min_stack.get_min(), -1);
    }

    #[test]
    fn test_min_updates() {
        let mut min_stack = MinStack::new();
        min_stack.push(5);
        assert_eq!(min_stack.get_min(), 5);

        min_stack.push(3);
        assert_eq!(min_stack.get_min(), 3);

        min_stack.push(7);
        assert_eq!(min_stack.get_min(), 3);

        min_stack.pop(); // Remove 7
        assert_eq!(min_stack.get_min(), 3);

        min_stack.pop(); // Remove 3
        assert_eq!(min_stack.get_min(), 5);
    }
}
