#![forbid(unsafe_code)]

use std::collections::VecDeque;

#[derive(Default)]
pub struct MinQueue<T> {
    elems_queue: VecDeque<T>,
    mins_queue: VecDeque<T>,
}

impl<T: Clone + Ord> MinQueue<T> {
    pub fn new() -> Self {
        Self {
            elems_queue: VecDeque::<T>::new(),
            mins_queue: VecDeque::<T>::new(),
        }
    }

    pub fn push(&mut self, val: T) {
        loop {
            match self.mins_queue.back() {
                Some(min) if min > &val => {
                    self.mins_queue.pop_back();
                }
                _ => break,
            }
        }

        self.mins_queue.push_back(val.clone());
        self.elems_queue.push_back(val);
    }

    pub fn pop(&mut self) -> Option<T> {
        if self.elems_queue.front() == self.mins_queue.front() {
            self.mins_queue.pop_front();
        }

        self.elems_queue.pop_front()
    }

    pub fn front(&self) -> Option<&T> {
        self.elems_queue.front()
    }

    pub fn min(&self) -> Option<&T> {
        self.mins_queue.front()
    }

    pub fn len(&self) -> usize {
        self.elems_queue.len()
    }

    pub fn is_empty(&self) -> bool {
        self.elems_queue.is_empty()
    }
}
