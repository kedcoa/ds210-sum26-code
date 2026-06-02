use rand::rng;
use rand::seq::SliceRandom;

pub fn random_vector(n: usize) -> (Vec<i32>, i32) {
    let mut v: Vec<i32> = (0..n as i32).collect();
    v.shuffle(&mut rng());
    let removed = v.pop().unwrap();
    return (v, removed);
}