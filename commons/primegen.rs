pub struct PrimeGen {
    primes: Vec<u64>,
    n: u64
}


impl PrimeGen {
    pub fn new() -> PrimeGen {
        PrimeGen {
            primes: Vec::with_capacity(2000000),
            n: 2
        }
    }
}

impl Iterator<u64> for PrimeGen {
    fn next(&mut self) -> Option<u64> {
        loop {
            let mut is_prime = true;
            for prime in self.primes.iter() {
                if self.n % *prime == 0 {
                    is_prime = false;
                    break;
                }
            }
            if is_prime {
                self.primes.push(self.n);
                return Some(self.n);
            }
            self.n += 1;
        }
    }
}