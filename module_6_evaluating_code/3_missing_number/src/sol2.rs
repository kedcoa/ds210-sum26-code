use std::time::Instant;

mod helpers;

// https://leetcode.com/problems/missing-number/description/
pub fn missing_number_overflow(nums: Vec<i32>) -> i32 {
    let n = nums.len() as i32;
    let expected_sum: i32 = n * (n + 1) / 2;
    let actual_sum: i32 = nums.iter().sum();
    return expected_sum - actual_sum;
}

pub fn missing_number_correct(nums: Vec<i32>) -> i32 {
    let n = nums.len() as i64;
    let expected_sum: i64 = n * (n + 1) / 2;
    let mut actual_sum: i64 = 0;
    for n in nums {
        actual_sum += n as i64;
    }
    return (expected_sum - actual_sum) as i32;
}

fn main() {
    let (v, missing) = helpers::random_vector(1_000_000);
    let time = Instant::now();
    let number = missing_number_correct(v);
    let duration = time.elapsed();
    assert_eq!(number, missing);
    println!("Answer is {}", number);
    println!("Took {:?}", duration);
}