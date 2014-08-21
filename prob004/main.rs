struct ProductIterator {
    left_vec: Vec<u64>,
    left_idx: uint,
    right_vec: Vec<u64>,
    right_idx: uint
}


impl ProductIterator {
    fn new(left: Vec<u64>, right: Vec<u64>) -> ProductIterator {
        ProductIterator {
            left_vec: left,
            left_idx: 0,
            right_vec: right,
            right_idx: 0
        }
    }
}

impl Iterator<(u64, u64)> for ProductIterator {
    fn next(&mut self) -> Option<(u64, u64)> {
        if self.right_vec.len() <= self.right_idx {
            self.left_idx += 1;
            self.right_idx = 0;
        }
        if self.left_vec.len() <= self.left_idx {
            None
        } else {
            let out = (
                self.left_vec[self.left_idx],
                self.right_vec[self.right_idx]
            );
            self.right_idx += 1;
            Some(out)
        }
        
    }
}


fn is_palindrome(val: u64) -> bool {
    let digits = format!("{}", val).into_bytes();
    for (a, b) in digits.iter().zip(digits.iter().rev()) {
        if a != b {
            return false;
        }
    }
    true
}


fn main() {
    let iterator = ProductIterator::new(
        range(100, 1000).collect::<Vec<u64>>(),
        range(100, 1000).collect::<Vec<u64>>());

    let value = iterator
                .map(|f| { let (a, b) = f; a * b })
                .filter(|&x| is_palindrome(x))
                .max()
                .unwrap();

    println!("result = {}", value);
}


#[test]
fn test_euler_result() {
    let iterator = ProductIterator::new(
        range(10, 100).collect::<Vec<u64>>(),
        range(10, 100).collect::<Vec<u64>>());
    let value = iterator
                .map(|f| { let (a, b) = f; a * b })
                .filter(|&x| is_palindrome(x))
                .max()
                .unwrap();
    assert_eq!(9009, value);
}


#[test]
fn test_length_correctness() {
    let iterator = ProductIterator::new(
        range(100, 1000).collect::<Vec<u64>>(),
        range(100, 1000).collect::<Vec<u64>>());

    assert_eq!(810000, iterator.map(|_| 1_u64).sum());
}