use std::iter::AdditiveIterator;


struct FibonacciIterator {
    prev: u32,
    cur: u32,
    finished: bool
}


impl FibonacciIterator {
    pub fn new() -> FibonacciIterator {
        FibonacciIterator {
            prev: 0,
            cur: 1,
            finished: false
        }
    }
}


impl Iterator<u32> for FibonacciIterator {
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
    let sum = FibonacciIterator::new()
                .take_while(|&x| x < 4000000)
                .filter(|&x| x % 2 == 0)
                .sum();
    println!("sum = {}", sum);
}
