pub struct ProductIterator<A, B> {
    left_vec: Vec<A>,
    left_idx: uint,
    right_vec: Vec<B>,
    right_idx: uint,
}


impl<A, B> ProductIterator<A, B> {
    pub fn new(left: Vec<A>, right: Vec<B>) -> ProductIterator<A, B> {
        ProductIterator {
            left_vec: left,
            left_idx: 0,
            right_vec: right,
            right_idx: 0
        }
    }
}

impl<A: Clone, B: Clone> Iterator<(A, B)> for ProductIterator<A, B> {
    fn next(&mut self) -> Option<(A, B)> {
        if self.right_vec.len() <= self.right_idx {
            self.left_idx += 1;
            self.right_idx = 0;
        }
        if self.left_vec.len() <= self.left_idx {
            None
        } else {
            let out = (
                self.left_vec[self.left_idx].clone(),
                self.right_vec[self.right_idx].clone()
            );
            self.right_idx += 1;
            Some(out)
        }
        
    }
}