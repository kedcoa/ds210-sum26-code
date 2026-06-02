mod helpers;

pub fn find_kth_largest(mut nums: Vec<i32>, k: usize) -> i32 {
    quickselect(&mut nums, k)
}

use rand::Rng;

fn quickselect(nums: &mut [i32], k: usize) -> i32 {
    let n = nums.len();
    let pivot_idx = rand::thread_rng().gen_range(0..n);
    let pivot = nums[pivot_idx];
    nums.swap(pivot_idx, n - 1);

    // partition: elements strictly greater than pivot go right
    let mut i = 0;
    for j in 0..n - 1 {
        if nums[j] < pivot {
            nums.swap(i, j);
            i += 1;
        }
    }
    nums.swap(i, n - 1);
    // The following is now true:
    // 1. nums[0..i] < pivot
    // 2. nums[i] == pivot
    // 3. nums[i+1..] > pivot

    // the rank of the pivot (1 -> pivot is the largest element, x means pivot is the xth largest element)
    let pivot_rank = n - i;
    if k == pivot_rank {
        pivot
    } else if k < pivot_rank {
        quickselect(&mut nums[i + 1..], k)
    } else {
        quickselect(&mut nums[..i], k - pivot_rank)
    }
}

fn main() {
    let (nums, k, expected) = helpers::random_vector(10_000_000);
    let start = std::time::Instant::now();
    let result = find_kth_largest(nums, k);
    let elapsed = start.elapsed();
    assert_eq!(result, expected);
    println!("result: {}, time: {:?}", result, elapsed);
}