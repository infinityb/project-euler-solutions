pub struct UniqIterator<A, B> {
    prev_val: Option<A>,
    iterator: B
}


impl<A: PartialEq + Clone, B: Iterator<A>> UniqIterator<A, B> {
    fn new(iterator: B) -> UniqIterator<A, B> {
        UniqIterator {
            prev_val: None,
            iterator: iterator
        }
    }
}


impl<A: PartialEq + Clone, B: Iterator<A>> Iterator<A> for UniqIterator<A, B> {
    fn next(&mut self) -> Option<A> {
        loop {
            let cur_val = match self.iterator.next() {
                Some(val) => val,
                None => return None
            };
            match self.prev_val.clone() {
                Some(prev_val) => {
                    if prev_val != cur_val {
                        self.prev_val = Some(cur_val.clone());
                        return Some(cur_val);
                    }  // else continue
                },
                None => {
                    // initialisation phase
                    self.prev_val = Some(cur_val.clone());
                    return Some(cur_val);
                }
            }
        }
    }
}


pub fn uniq<A: PartialEq + Clone, B: Iterator<A>>(iterator: B) -> UniqIterator<A, B> {
    UniqIterator::new(iterator)
}


#[test]
fn test_uniq_iter() {
    let nums = vec![0_u32, 0, 0, 1, 2, 3, 4, 4, 4, 5, 6, 6, 7, 7, 8, 9];
    let expected = vec![0_u32, 1, 2, 3, 4, 5, 6, 7, 8, 9];
    let got = uniq(nums.move_iter()).collect::<Vec<u32>>();
    assert_eq!(expected, got);

    let nums: Vec<u32> = vec![];
    let expected: Vec<u32> = vec![];
    let got = uniq(nums.move_iter()).collect::<Vec<u32>>();
    assert_eq!(expected, got);

}