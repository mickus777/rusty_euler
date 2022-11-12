
pub struct TriangleNumbers {
    current_sum: u64,
    last_term: u64
}

impl TriangleNumbers {
    pub fn new() -> TriangleNumbers {
        TriangleNumbers { current_sum: 0, last_term: 0 }
    }

    pub fn next(&mut self) -> u64 {
        self.last_term += 1;
        self.current_sum += self.last_term;
        self.current_sum
    }
}