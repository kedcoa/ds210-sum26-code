use rand::Rng;
use rand::seq::SliceRandom;

pub fn random_vector(n: usize) -> (Vec<i32>, usize, i32) {
    let mut rng = rand::thread_rng();
    let mut v: Vec<i32> = (0..n as i32).collect();
    let k = rng.gen_range(1..=n);
    let kth_largest = v[n - k];
    v.shuffle(&mut rng);
    (v, k, kth_largest)
}