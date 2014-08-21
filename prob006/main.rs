use std::iter::AdditiveIterator;
use std::num::pow;


#[cfg(not(test))]
fn main() {
    let sum_of_sqrs = range(0_u64, 101).map(|x| pow(x, 2)).sum();
    let sqr_of_sums = pow(range(0_u64, 101).sum(), 2);
    println!("result = {}", sqr_of_sums - sum_of_sqrs);
}


#[test]
fn test_euler_result() {
    let sum_of_sqrs = range(0_u64, 11).map(|x| pow(x, 2)).sum();
    let sqr_of_sums = pow(range(0_u64, 11).sum(), 2);
    assert_eq!(sqr_of_sums - sum_of_sqrs, 2640);
}