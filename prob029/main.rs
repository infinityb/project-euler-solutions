extern crate commons;
extern crate num;

use std::num::{pow, FromPrimitive};
use std::iter::{range_inclusive, AdditiveIterator};
use num::bigint::BigInt;

use commons::{uniq, ProductIterator};


fn iterlen<A, B: Iterator<A>>(a: B) -> uint {
    a.map(|_| 1 as uint).sum()
}

#[cfg(not(test))]
fn main() {
    let big2: BigInt = FromPrimitive::from_int(2).unwrap();
    let big100: BigInt = FromPrimitive::from_int(100).unwrap();

    let proditer = ProductIterator::new(
        range_inclusive(big2.clone(), big100.clone()).collect::<Vec<BigInt>>(),
        range_inclusive(2, 100).collect::<Vec<uint>>());

    let mut got = proditer.map(|(a, b)| pow(a, b)).collect::<Vec<BigInt>>();
    got.sort();
    println!("len = {}", iterlen(uniq(got.into_iter())));
}


#[test]
fn test_euler_result() {
    let big2: BigInt = FromPrimitive::from_int(2).unwrap();
    let big5: BigInt = FromPrimitive::from_int(5).unwrap();

    let proditer = ProductIterator::new(
        range_inclusive(big2, big5).collect::<Vec<BigInt>>(),
        range_inclusive(2, 5).collect::<Vec<uint>>());

    let mut got = proditer.map(|(a, b)| pow(a, b)).collect::<Vec<BigInt>>();
    got.sort();
    assert_eq!(iterlen(uniq(got.into_iter())), 15);
}
