extern crate num;

use std::num::{pow, FromPrimitive, Zero};
use num::bigint::BigInt;


fn weird(value: BigInt) -> u64 {
    let big10: BigInt = FromPrimitive::from_int(10).unwrap();
    let mut number = value.clone();
    let mut acc = 0_u64;
    while number > Zero::zero() {
        acc += (number % big10).to_u64().unwrap();
        number = number / big10;
    }
    acc
}


#[cfg(not(test))]
fn main() {
    let big2: BigInt = FromPrimitive::from_int(2).unwrap();
    println!("result: {}", weird(pow(big2, 1000)));
}


#[test]
fn test_euler_result() {
    let big2: BigInt = FromPrimitive::from_int(2).unwrap();
    assert_eq!(weird(pow(big2, 15)), 26);
}