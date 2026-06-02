use std::collections::HashSet;
use std::time::Instant;

mod helpers;

// https://leetcode.com/problems/missing-number/description/
pub fn missing_number(nums: Vec<i32>) -> i32 {
    let n = nums.len() as i32;

    let mut set = HashSet::new();
    for num in nums {
        set.insert(num);
    }

    for num in 0..n {
        if !set.contains(&num) {
            return num;
        }
    }
    return n;
}

fn main() {
    let (v, missing) = helpers::random_vector(10_000);
    let time = Instant::now();
    let number = missing_number(v);
    let duration = time.elapsed();
    assert_eq!(number, missing);
    println!("Answer is {}", number);
    println!("Took {:?}", duration);
}
