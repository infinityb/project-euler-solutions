extern crate commons;

use std::num::pow;
use commons::ProductIterator;


#[cfg(not(test))]
fn main() {
}


#[test]
fn x() {
    let expect = vec![
        4, 8, 9, 16, 25, 27, 32, 64, 81, 125, 243, 256, 625, 1024, 3125];
    let got = ProductIterator::new(
        range(2, 6).collect::<Vec<u64>>(),
        range(2, 6).collect::<Vec<uint>>());
    assert_eq!(got.map(|(a, b)| pow(a, b)).uniq().collect(), expect);
}