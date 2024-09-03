#![forbid(unsafe_code)]

use std::{cell::RefCell, collections::VecDeque, mem::swap, rc::Rc};

pub struct LazyCycle<I: Iterator> {
    iter: I,
    elems: Vec<I::Item>,
    is_iter_exhausted: bool,
    index: usize,
}

impl<I> LazyCycle<I>
where
    I: Iterator,
{
    fn new(iter: I) -> Self {
        Self {
            iter,
            elems: vec![],
            is_iter_exhausted: false,
            index: 0,
        }
    }
}

impl<I> Iterator for LazyCycle<I>
where
    I: Iterator,
    I::Item: Clone,
{
    type Item = I::Item;

    fn next(&mut self) -> Option<Self::Item> {
        if !self.is_iter_exhausted {
            let elem = self.iter.next();

            match elem {
                Some(x) => {
                    self.elems.push(x.clone());
                    Some(x)
                }
                None => {
                    self.is_iter_exhausted = true;
                    if self.elems.is_empty() {
                        return None;
                    }

                    let elem = self.elems[self.index].clone();
                    self.index = (self.index + 1) % self.elems.len();
                    Some(elem)
                }
            }
        } else {
            if self.elems.is_empty() {
                return None;
            }

            let elem = self.elems[self.index].clone();
            self.index = (self.index + 1) % self.elems.len();
            Some(elem)
        }
    }
}

////////////////////////////////////////////////////////////////////////////////

pub struct Extract<I: Iterator> {
    elems: VecDeque<I::Item>,
    iter: I,
}

impl<I: Iterator> Extract<I> {
    fn new(elems: VecDeque<I::Item>, iter: I) -> Self {
        Self { elems, iter }
    }
}

impl<I: Iterator> Iterator for Extract<I> {
    type Item = I::Item;

    fn next(&mut self) -> Option<Self::Item> {
        if !self.elems.is_empty() {
            Some(self.elems.pop_front().unwrap())
        } else {
            self.iter.next()
        }
    }
}

////////////////////////////////////////////////////////////////////////////////

struct TeeBuffer<Item, I> {
    elems: VecDeque<Item>,
    iter: I,
    owner_id: u8,
    is_iter_exhausted: bool,
}

pub struct Tee<I>
where
    I: Iterator,
    I::Item: Clone,
{
    buffer: Rc<RefCell<TeeBuffer<I::Item, I>>>,
    id: u8,
}

impl<I> Tee<I>
where
    I: Iterator,
    I::Item: Clone,
{
    fn new(iter: I) -> (Self, Self) {
        let buffer = TeeBuffer {
            elems: VecDeque::new(),
            iter,
            owner_id: 1,
            is_iter_exhausted: false,
        };
        let tee_1 = Tee {
            buffer: Rc::new(RefCell::new(buffer)),
            id: 0,
        };
        let tee_2 = Tee {
            buffer: tee_1.buffer.clone(),
            id: 1,
        };
        (tee_1, tee_2)
    }
}

impl<I> Iterator for Tee<I>
where
    I: Iterator,
    I::Item: Clone,
{
    type Item = I::Item;

    fn next(&mut self) -> Option<Self::Item> {
        let mut buffer = self.buffer.borrow_mut();

        if buffer.owner_id == self.id {
            if let Some(x) = buffer.elems.pop_front() {
                return Some(x);
            }
        }

        if buffer.is_iter_exhausted {
            return None;
        }

        match buffer.iter.next() {
            Some(x) => {
                buffer.elems.push_back(x.clone());
                buffer.owner_id = 1 - self.id;
                Some(x)
            }
            None => {
                buffer.is_iter_exhausted = true;
                None
            }
        }
    }
}

////////////////////////////////////////////////////////////////////////////////

pub struct GroupBy<I, F, V>
where
    I: Iterator,
    F: FnMut(&I::Item) -> V,
    V: Eq,
{
    iter: I,
    func: F,
    previous: Option<I::Item>,
}

impl<I, F, V> GroupBy<I, F, V>
where
    I: Iterator,
    F: FnMut(&I::Item) -> V,
    V: Eq,
{
    fn new(iter: I, func: F) -> Self {
        Self {
            iter,
            func,
            previous: None,
        }
    }
}

impl<I, F, V> Iterator for GroupBy<I, F, V>
where
    I: Iterator,
    F: FnMut(&I::Item) -> V,
    V: Eq,
{
    type Item = (V, Vec<I::Item>);

    fn next(&mut self) -> Option<(V, Vec<I::Item>)> {
        let mut group = Vec::<I::Item>::new();
        let key: V;
        let mut previous = None;
        swap(&mut previous, &mut self.previous);

        match previous {
            Some(x) => {
                key = (self.func)(&x);
                group.push(x);
            }
            None => {
                let elem = self.iter.next();

                elem.as_ref()?;

                key = (self.func)(elem.as_ref().unwrap());
                group.push(elem.unwrap());
            }
        }

        loop {
            let elem = self.iter.next();

            if elem.is_none() {
                break;
            }

            if (self.func)(elem.as_ref().unwrap()) != key {
                self.previous = elem;
                break;
            }

            group.push(elem.unwrap());
        }

        Some((key, group))
    }
}

////////////////////////////////////////////////////////////////////////////////

pub trait ExtendedIterator: Iterator {
    fn lazy_cycle(self) -> LazyCycle<Self>
    where
        Self: Sized,
    {
        LazyCycle::new(self)
    }

    fn extract(mut self, index: usize) -> (Option<Self::Item>, Extract<Self>)
    where
        Self: Sized,
    {
        let mut elems = VecDeque::new();
        let mut elem_index = 0;
        let mut value = None;

        loop {
            let elem = self.next();

            if elem.is_none() {
                break;
            }

            if elem_index == index {
                value = Some(elem.unwrap());
                break;
            } else {
                elems.push_back(elem.unwrap());
            }

            elem_index += 1;
        }

        (value, Extract::<Self>::new(elems, self))
    }

    fn tee(self) -> (Tee<Self>, Tee<Self>)
    where
        Self: Sized,
        Self::Item: Clone,
    {
        Tee::new(self)
    }

    fn group_by<F, V>(self, func: F) -> GroupBy<Self, F, V>
    where
        Self: Sized,
        F: FnMut(&Self::Item) -> V,
        V: Eq,
    {
        GroupBy::<Self, F, V>::new(self, func)
    }
}

impl<T> ExtendedIterator for T where T: Iterator {}
