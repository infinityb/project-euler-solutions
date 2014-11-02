use std::iter::Peekable;

pub struct SortedUnion<A, T, R> {
    iterator_a: Peekable<A, T>,
    iterator_b: Peekable<A, R>,
}


impl<A, T: Iterator<A>, R: Iterator<A>> SortedUnion<A, T, R> {
    fn new(iterator_a: T, iterator_b: R) -> SortedUnion<A, T, R> {
        SortedUnion {
            iterator_a: iterator_a.peekable(),
            iterator_b: iterator_b.peekable()
        }
    }
}


impl<A: Ord + Clone, T: Iterator<A>, R: Iterator<A>> Iterator<A> for SortedUnion<A, T, R> {
    fn next(&mut self) -> Option<A> {
        let b_peek = match self.iterator_b.peek() {
            Some(b_peek) => b_peek.clone(),
            None => return self.iterator_a.next()
        };
        let a_peek = match self.iterator_a.peek() {
            Some(a_peek) => a_peek.clone(),
            None => return self.iterator_b.next()
        };
        match a_peek.cmp(&b_peek) {
            Less => self.iterator_a.next(),
            Equal => self.iterator_a.next(),
            Greater => self.iterator_b.next()
        }
    }
}


pub fn sorted_union<A: Ord + Clone, T: Iterator<A>, R: Iterator<A>>(
        iterator_a: T, iterator_b: R) -> SortedUnion<A, T, R> {
    SortedUnion::new(iterator_a, iterator_b)
}


#[test]
fn test_sorted_union_easy() {
    let expected: Vec<u32> = vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9];

    let nums_a: Vec<u32> = vec![1, 3, 5, 7, 9];
    let nums_b: Vec<u32> = vec![0, 2, 4, 6, 8];
    let got: Vec<u32> = SortedUnion::new(nums_a.into_iter(), nums_b.into_iter()).collect();
    assert_eq!(got, expected);

    let nums_a: Vec<u32> = vec![1, 3, 5, 7, 9];
    let nums_b: Vec<u32> = vec![0, 2, 4, 6, 8];
    let got: Vec<u32> = SortedUnion::new(nums_b.into_iter(), nums_a.into_iter()).collect();
    assert_eq!(got, expected);
}


#[test]
fn test_sorted_union_with_dups() {
    let expected: Vec<u32> = vec![0, 1, 2, 3, 4, 4, 5, 6, 7, 8, 9];

    let nums_a: Vec<u32> = vec![1, 3, 5, 7, 9];
    let nums_b: Vec<u32> = vec![0, 2, 4, 4, 6, 8];
    let got: Vec<u32> = SortedUnion::new(nums_a.into_iter(), nums_b.into_iter()).collect();
    assert_eq!(got, expected); 

    let nums_a: Vec<u32> = vec![1, 3, 5, 7, 9];
    let nums_b: Vec<u32> = vec![0, 2, 4, 4, 6, 8];
    let got: Vec<u32> = SortedUnion::new(nums_b.into_iter(), nums_a.into_iter()).collect();
    assert_eq!(got, expected);

    let nums_a: Vec<u32> = vec![1, 3, 4, 5, 7, 9];
    let nums_b: Vec<u32> = vec![0, 2, 4, 6, 8];
    let got: Vec<u32> = SortedUnion::new(nums_a.into_iter(), nums_b.into_iter()).collect();
    assert_eq!(got, expected);

    let nums_a: Vec<u32> = vec![1, 3, 4, 5, 7, 9];
    let nums_b: Vec<u32> = vec![0, 2, 4, 6, 8];
    let got: Vec<u32> = SortedUnion::new(nums_b.into_iter(), nums_a.into_iter()).collect();
    assert_eq!(got, expected);
}


#[test]
fn test_sorted_union_empty() {
    let expected: Vec<u32> = vec![];

    let nums_a: Vec<u32> = vec![];
    let nums_b: Vec<u32> = vec![];
    let got: Vec<u32> = SortedUnion::new(nums_a.into_iter(), nums_b.into_iter()).collect();
    assert_eq!(got, expected);
}