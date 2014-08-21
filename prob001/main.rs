use std::iter::AdditiveIterator;


#[cfg(not(test))]
fn main() {
    let sum = range(0_u32, 1000).filter(|&x| x % 3 == 0 || x % 5 == 0).sum();
    println!("sum = {}", sum);
}


#[test]
fn test_euler_result() {
	let sum = range(0_u32, 10).filter(|&x| x % 3 == 0 || x % 5 == 0).sum();
	assert_eq!(sum, 23);
}
