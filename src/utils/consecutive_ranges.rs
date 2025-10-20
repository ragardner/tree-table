pub struct ConsecutiveRangesBorrowed<'a, I>
where
    I: Iterator<Item = &'a usize>,
{
    iter: I,
    start: Option<usize>,
    prev: Option<usize>,
}

impl<'a, I> Iterator for ConsecutiveRangesBorrowed<'a, I>
where
    I: Iterator<Item = &'a usize>,
{
    type Item = (usize, usize);

    fn next(&mut self) -> Option<Self::Item> {
        // Initialize current_start and current_prev from self.start and self.prev
        let (current_start, mut current_prev) = match (self.start, self.prev) {
            (Some(start), Some(prev)) => (start, prev),
            (None, None) => {
                if let Some(&first) = self.iter.next() {
                    self.start = Some(first);
                    self.prev = Some(first);
                    (first, first)
                } else {
                    return None;
                }
            }
            _ => unreachable!("start and prev should both be Some or both be None"),
        };

        // Process the iterator
        loop {
            match self.iter.next() {
                Some(&curr) => {
                    if curr > current_prev + 1 {
                        // Gap found: return the range and update state
                        let range = (current_start, current_prev + 1);
                        self.start = Some(curr);
                        self.prev = Some(curr);
                        return Some(range);
                    } else {
                        // Continue the current range
                        current_prev = curr;
                        self.prev = Some(curr);
                    }
                }
                None => {
                    // End of iterator: return the final range and reset
                    let range = (current_start, current_prev + 1);
                    self.start = None;
                    self.prev = None;
                    return Some(range);
                }
            }
        }
    }
}

/// Iterator of tuples (usize, usize) from an iterable of sorted positive usize.
/// Each yielded tuple is a (start, exclusive end) of a consecutive range.
pub fn consecutive_ranges_borrowed<'a, I>(iter: I) -> ConsecutiveRangesBorrowed<'a, I>
where
    I: Iterator<Item = &'a usize>,
{
    ConsecutiveRangesBorrowed {
        iter,
        start: None,
        prev: None,
    }
}

pub struct ConsecutiveRangesOwned<I>
where
    I: Iterator<Item = usize>,
{
    iter: I,
    start: Option<usize>,
    prev: Option<usize>,
}

impl<I> Iterator for ConsecutiveRangesOwned<I>
where
    I: Iterator<Item = usize>,
{
    type Item = (usize, usize);

    fn next(&mut self) -> Option<Self::Item> {
        let (current_start, mut current_prev) = match (self.start, self.prev) {
            (Some(start), Some(prev)) => (start, prev),
            (None, None) => {
                if let Some(first) = self.iter.next() {
                    self.start = Some(first);
                    self.prev = Some(first);
                    (first, first)
                } else {
                    return None;
                }
            }
            _ => unreachable!("start and prev should both be Some or both be None"),
        };

        loop {
            match self.iter.next() {
                Some(curr) => {
                    if curr > current_prev + 1 {
                        let range = (current_start, current_prev + 1);
                        self.start = Some(curr);
                        self.prev = Some(curr);
                        return Some(range);
                    } else {
                        current_prev = curr;
                        self.prev = Some(curr);
                    }
                }
                None => {
                    let range = (current_start, current_prev + 1);
                    self.start = None;
                    self.prev = None;
                    return Some(range);
                }
            }
        }
    }
}

/// Iterator of tuples (usize, usize) from an iterable of sorted positive usize.
/// Each yielded tuple is a (start, exclusive end) of a consecutive range.
/// This version takes an iterator yielding owned usize values.
pub fn consecutive_ranges_owned<I>(iter: I) -> ConsecutiveRangesOwned<I>
where
    I: Iterator<Item = usize>,
{
    ConsecutiveRangesOwned {
        iter,
        start: None,
        prev: None,
    }
}
