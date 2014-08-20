// The prime factors of 13195 are 5, 7, 13 and 29.
// What is the largest prime factor of the number 600851475143 ?


struct SimpleFactoringIterator {
    remaining: u64
}


impl Iterator<u64> for SimpleFactoringIterator {
    fn next(&mut self) -> Option<u64> {
        for cand_factor in range(2, self.remaining + 1) {
            if self.remaining % cand_factor == 0 {
                self.remaining = self.remaining / cand_factor;
                return Some(cand_factor);
            }
        }
        None
    }
}


fn factor(num: u64) -> SimpleFactoringIterator {
    SimpleFactoringIterator {
        remaining: num
    }
}

fn main() {
    let mut biggest = 0;
    for prime_factor in factor(600851475143) {
        biggest = prime_factor;
    }
    println!("biggest prime factor = {}", biggest);
}


#[test]
fn test_euler_result() {
    let factors: Vec<u64> = factor(13195).collect();
    assert_eq!(factors, vec![5, 7, 13, 29]);
}
