#[derive(Debug, Clone)]
pub struct CyclingCounter {
    current: usize,
}

impl CyclingCounter {
    /// Creates a new `CyclingCounter` starting at 0.
    pub fn new() -> Self {
        CyclingCounter { current: 0 }
    }
}

impl Iterator for CyclingCounter {
    type Item = usize;

    /// Returns the next `usize` value, cycling back to 0 after `usize::MAX`.
    fn next(&mut self) -> Option<Self::Item> {
        let current = self.current;
        self.current = self.current.wrapping_add(1);
        Some(current)
    }
}
