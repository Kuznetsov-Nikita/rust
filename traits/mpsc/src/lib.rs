#![forbid(unsafe_code)]

use std::{cell::RefCell, collections::VecDeque, fmt::Debug, rc::Rc};
use thiserror::Error;

////////////////////////////////////////////////////////////////////////////////

// TODO: your code goes here.

////////////////////////////////////////////////////////////////////////////////

#[derive(Error, Debug)]
#[error("channel is closed")]
pub struct SendError<T> {
    pub value: T,
}

pub struct Sender<T> {
    data: Rc<RefCell<VecDeque<T>>>,
    is_close: Rc<RefCell<bool>>,
}

impl<T> Sender<T> {
    pub fn send(&self, value: T) -> Result<(), SendError<T>> {
        if self.is_closed() {
            return Err(SendError { value });
        }

        self.data.borrow_mut().push_front(value);
        Ok(())
    }

    pub fn is_closed(&self) -> bool {
        *self.is_close.borrow()
    }

    pub fn same_channel(&self, other: &Self) -> bool {
        self.data.as_ptr() == other.data.as_ptr()
    }
}

impl<T> Clone for Sender<T> {
    fn clone(&self) -> Self {
        Self {
            data: self.data.clone(),
            is_close: self.is_close.clone(),
        }
    }
}

impl<T> Drop for Sender<T> {
    fn drop(&mut self) {
        if Rc::strong_count(&self.data) <= 2 {
            *self.is_close.borrow_mut() = true
        }
    }
}

////////////////////////////////////////////////////////////////////////////////

#[derive(Error, Debug)]
pub enum ReceiveError {
    #[error("channel is empty")]
    Empty,
    #[error("channel is closed")]
    Closed,
}

pub struct Receiver<T> {
    data: Rc<RefCell<VecDeque<T>>>,
    is_close: Rc<RefCell<bool>>,
}

impl<T> Receiver<T> {
    pub fn recv(&mut self) -> Result<T, ReceiveError> {
        if self.data.borrow().is_empty() {
            if *self.is_close.borrow() {
                return Err(ReceiveError::Closed);
            }
            return Err(ReceiveError::Empty);
        }

        Ok(self.data.borrow_mut().pop_back().unwrap())
    }

    pub fn close(&mut self) {
        *self.is_close.borrow_mut() = true
    }
}

impl<T> Drop for Receiver<T> {
    fn drop(&mut self) {
        self.close()
    }
}

////////////////////////////////////////////////////////////////////////////////

pub fn channel<T>() -> (Sender<T>, Receiver<T>) {
    let data = Rc::new(RefCell::new(VecDeque::new()));
    let is_close = Rc::new(RefCell::new(false));

    (
        Sender {
            data: data.clone(),
            is_close: is_close.clone(),
        },
        Receiver { data, is_close },
    )
}
