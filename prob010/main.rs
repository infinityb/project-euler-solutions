extern crate commons;

use std::iter::AdditiveIterator;
use commons::PrimeGen;


#[cfg(not(test))]
fn main() {
    println!("n = {}", PrimeGen::new().take_while(|&f| f < 2000000_u64).sum());
}


#[test]
fn test_euler_result() {
    let got = PrimeGen::new().take_while(|&f| f < 10_u64).sum();
    assert_eq!(got, 17);
}
