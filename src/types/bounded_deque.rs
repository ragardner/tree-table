use alloc::collections::VecDeque;

#[derive(Debug, Clone)]
pub struct BoundedDeque<T> {
    deque: VecDeque<T>,
    maxlen: usize,
}

impl<T> BoundedDeque<T> {
    /// Creates a new bounded deque with the given maximum length.
    pub fn new(maxlen: usize) -> Self {
        BoundedDeque {
            deque: VecDeque::new(),
            maxlen,
        }
    }

    /// Returns the current number of items in the deque.
    pub fn len(&self) -> usize {
        self.deque.len()
    }

    /// Sets a new maximum length for the deque.
    /// If the new maxlen is smaller than the current length,
    /// elements are removed from the start until the length is within the new limit.
    pub fn set_maxlen(&mut self, new_maxlen: usize) {
        self.maxlen = new_maxlen;
        while self.deque.len() > self.maxlen {
            self.deque.pop_front();
        }
    }

    /// Returns the current maximum length of the deque.
    pub fn get_maxlen(&self) -> usize {
        self.maxlen
    }

    /// Adds an item to the end. If the deque is at capacity, removes from the start first.
    pub fn push_back(&mut self, item: T) {
        if self.deque.len() == self.maxlen {
            self.deque.pop_front();
        }
        self.deque.push_back(item);
    }

    /// Adds an item to the start. If the deque is at capacity, removes from the end first.
    pub fn push_front(&mut self, item: T) {
        if self.deque.len() == self.maxlen {
            self.deque.pop_back();
        }
        self.deque.push_front(item);
    }

    /// Removes and returns the start item, if any.
    pub fn pop_front(&mut self) -> Option<T> {
        self.deque.pop_front()
    }

    /// Removes and returns the end item, if any.
    pub fn pop_back(&mut self) -> Option<T> {
        self.deque.pop_back()
    }

    /// Returns a reference to the start item, if any.
    pub fn front(&self) -> Option<&T> {
        self.deque.front()
    }

    /// Returns a reference to the end item, if any.
    pub fn back(&self) -> Option<&T> {
        self.deque.back()
    }

    /// Checks if the deque is empty.
    pub fn is_empty(&self) -> bool {
        self.deque.is_empty()
    }

    /// Clears the deque.
    pub fn clear(&mut self) {
        self.deque.clear();
    }

    /// Returns a reference to the last (most recently added) item, if any.
    pub fn last(&self) -> Option<&T> {
        self.deque.back()
    }

    /// Returns a mutable reference to the last (most recently added) item, if any.
    pub fn last_mut(&mut self) -> Option<&mut T> {
        self.deque.back_mut()
    }
}
