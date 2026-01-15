pub struct FibonacciSequence(pub usize);

pub struct FibonacciIter {
    current_index: usize,
    len: usize,
    prev: u64,
    before_prev: u64,
}

impl Iterator for FibonacciIter {
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item> {
        match self.current_index {
            0 => {
                self.current_index += 1;
                Some(0)
            }
            1 => {
                self.current_index += 1;
                Some(1)
            }
            n if n < self.len => {
                let nth = self.before_prev + self.prev;
                self.before_prev = self.prev;
                self.prev = nth;
                self.current_index += 1;
                Some(nth)
            }
            _ => None,
        }
    }
}

impl IntoIterator for FibonacciSequence {
    type Item = u64;
    type IntoIter = FibonacciIter;

    fn into_iter(self) -> Self::IntoIter {
        FibonacciIter {
            current_index: 0,
            len: self.0,
            prev: 1,
            before_prev: 0,
        }
    }
}
