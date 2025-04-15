struct MedianFinder {
    nums: Vec<i32>,
}

impl MedianFinder {
    fn new() -> Self {
        Self { nums: Vec::new() }
    }

    fn add_num(&mut self, num: i32) {
        // Insert number in the correct position to maintain sorted order
        // Using binary search to find the insertion position
        let pos = match self.nums.binary_search(&num) {
            Ok(pos) => pos,
            Err(pos) => pos, // If not found, returns where it would be inserted
        };
        self.nums.insert(pos, num);
    }

    fn find_median(&self) -> f64 {
        let len = self.nums.len();
        if len == 0 {
            return 0.0;
        }

        if len % 2 == 0 {
            // Even number of elements - average the middle two
            let mid = len / 2;
            (self.nums[mid - 1] as f64 + self.nums[mid] as f64) / 2.0
        } else {
            // Odd number of elements - return the middle one
            self.nums[len / 2] as f64
        }
    }
}

fn main() {
    // Run all tests
    test_empty();
    test_single_element();
    test_two_elements();
    test_odd_number_of_elements();
    test_even_number_of_elements();
    test_stream_of_numbers();

    println!("All tests passed!");
}

fn test_empty() {
    let finder = MedianFinder::new();
    assert_eq!(
        finder.find_median(),
        0.0,
        "Empty MedianFinder should return 0.0"
    );
    println!("✅ Empty test passed");
}

fn test_single_element() {
    let mut finder = MedianFinder::new();
    finder.add_num(5);
    assert_eq!(
        finder.find_median(),
        5.0,
        "Single element (5) median should be 5.0"
    );
    println!("✅ Single element test passed");
}

fn test_two_elements() {
    let mut finder = MedianFinder::new();
    finder.add_num(1);
    finder.add_num(3);
    assert_eq!(finder.find_median(), 2.0, "Median of [1, 3] should be 2.0");

    let mut finder2 = MedianFinder::new();
    finder2.add_num(7);
    finder2.add_num(7);
    assert_eq!(finder2.find_median(), 7.0, "Median of [7, 7] should be 7.0");

    println!("✅ Two elements test passed");
}

fn test_odd_number_of_elements() {
    let mut finder = MedianFinder::new();
    finder.add_num(1);
    finder.add_num(2);
    finder.add_num(3);
    assert_eq!(
        finder.find_median(),
        2.0,
        "Median of [1, 2, 3] should be 2.0"
    );

    finder.add_num(4);
    finder.add_num(5);
    assert_eq!(
        finder.find_median(),
        3.0,
        "Median of [1, 2, 3, 4, 5] should be 3.0"
    );

    println!("✅ Odd number of elements test passed");
}

fn test_even_number_of_elements() {
    let mut finder = MedianFinder::new();
    finder.add_num(1);
    finder.add_num(2);
    finder.add_num(3);
    finder.add_num(4);
    assert_eq!(
        finder.find_median(),
        2.5,
        "Median of [1, 2, 3, 4] should be 2.5"
    );

    finder.add_num(5);
    finder.add_num(6);
    assert_eq!(
        finder.find_median(),
        3.5,
        "Median of [1, 2, 3, 4, 5, 6] should be 3.5"
    );

    println!("✅ Even number of elements test passed");
}

fn test_stream_of_numbers() {
    let mut finder = MedianFinder::new();

    // Test adding numbers in unsorted order
    finder.add_num(41);
    assert_eq!(finder.find_median(), 41.0);

    finder.add_num(35);
    assert_eq!(finder.find_median(), 38.0);

    finder.add_num(62);
    assert_eq!(finder.find_median(), 41.0);

    finder.add_num(4);
    assert_eq!(finder.find_median(), 38.0);

    finder.add_num(97);
    assert_eq!(finder.find_median(), 41.0);

    finder.add_num(108);
    assert_eq!(finder.find_median(), 51.5);

    println!("✅ Stream of numbers test passed");
}
