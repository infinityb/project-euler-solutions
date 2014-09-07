extern crate commons;
use commons::PrimeGen;


#[cfg(not(test))]
fn main() {
    println!("10001st prime number: {}", PrimeGen::new().skip(10000).next());
}


#[test]
fn test_euler_result() {
	assert_eq!(PrimeGen::new().skip(5).next(), Some(13));
}