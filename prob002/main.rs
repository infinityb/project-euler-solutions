use std::iter::AdditiveIterator;


struct FibonacciItertor {
    prev: u32,
    cur: u32,
    finished: bool
}


impl FibonacciItertor {
    pub fn new() -> FibonacciItertor {
        FibonacciItertor {
            prev: 0,
            cur: 1,
            finished: false
        }
    }
}


impl Iterator<u32> for FibonacciItertor {
    fn next(&mut self) -> Option<u32> {
        if self.finished {
            return None;
        }
        let emit_value = self.cur;
        let (prev, cur) = (self.cur, self.prev.checked_add(&self.cur));

        match cur {
            Some(cur) => {
                self.prev = prev;
                self.cur = cur;
            },
            None => self.finished = true
        }
        Some(emit_value)
    }
}


fn main() {
    let sum = FibonacciItertor::new()
                .take_while(|&x| x < 4000000)
                .filter(|&x| x % 2 == 0)
                .sum();
    println!("sum = {}", sum);
}