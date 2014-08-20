use std::iter::AdditiveIterator;


fn main() {
    let sum = range(0_u32, 1000).filter(|&x| x % 3 == 0 || x % 5 == 0).sum();
    println!("sum = {}", sum);
}