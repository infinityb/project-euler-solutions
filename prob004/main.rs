extern crate commons;

#[cfg(test)]
use std::iter::AdditiveIterator;

use commons::ProductIterator;


fn is_palindrome(val: u64) -> bool {
    let digits = format!("{}", val).into_bytes();
    for (a, b) in digits.iter().zip(digits.iter().rev()) {
        if a != b {
            return false;
        }
    }
    true
}


#[cfg(not(test))]
fn main() {
    let iterator = ProductIterator::new(
        range(100, 1000).collect::<Vec<u64>>(),
        range(100, 1000).collect::<Vec<u64>>());

    let value = iterator
                .map(|f| { let (a, b) = f; a * b })
                .filter(|&x| is_palindrome(x))
                .max()
                .unwrap();

    println!("result = {}", value);
}


#[test]
fn test_euler_result() {
    let iterator = ProductIterator::new(
        range(10, 100).collect::<Vec<u64>>(),
        range(10, 100).collect::<Vec<u64>>());
    let value = iterator
                .map(|f| { let (a, b) = f; a * b })
                .filter(|&x| is_palindrome(x))
                .max()
                .unwrap();
    assert_eq!(9009, value);
}


#[test]
fn test_length_correctness() {
    let iterator = ProductIterator::new(
        range(100, 1000).collect::<Vec<u64>>(),
        range(100, 1000).collect::<Vec<u64>>());

    assert_eq!(810000, iterator.map(|_| 1_u64).sum());
}