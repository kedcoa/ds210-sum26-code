mod helpers;

use std::cmp::Reverse;
use std::collections::BinaryHeap;

pub fn find_kth_largest(nums: Vec<i32>, k: usize) -> i32 {
    let mut heap: BinaryHeap<Reverse<i32>> = BinaryHeap::new();
    for n in nums {
        heap.push(Reverse(n));
        if heap.len() > k {
            heap.pop();
        }
    }
    heap.peek().unwrap().0
}

fn main() {
    let (nums, k, expected) = helpers::random_vector(10_000_000);
    let start = std::time::Instant::now();
    let result = find_kth_largest(nums, k);
    let elapsed = start.elapsed();
    assert_eq!(result, expected);
    println!("result: {}, time: {:?}", result, elapsed);
}