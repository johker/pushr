use std::fmt;

#[derive(Debug)]
pub enum BufferType {
    Queue,
    Stack,
}

/// https://github.com/stjepangolemac/ringvec

#[derive(Debug)]
pub struct PushBuffer<T> {
    capacity: usize,
    container: Vec<T>,
    start: usize,
    end: usize,
    len: usize,
    buffer_type: BufferType,
}

impl<T> PushBuffer<T>
where
    T: Clone + fmt::Display + Default + PartialEq + fmt::Debug
{
    pub fn new(buffer_type: BufferType, capacity: usize) -> Self {
        let mut container = Vec::with_capacity(capacity);

        for _ in 0..capacity {
            container.push(T::default());
        }

        Self {
            capacity,
            container,
            start: 0,
            end: 0,
            len: 0,
            buffer_type,
        }
    }

    pub fn capacity(&self) -> usize {
        self.capacity
    }
    pub fn size(&self) -> usize {
        self.len
    }

    pub fn copy_oldest(&self) -> Option<T> {
        if !self.is_empty() {
            Some(self.container[self.end].clone())
        } else {
            None
        }

    }

    pub fn flush(&mut self)  {
        let capacity = self.capacity;
        self.container = Vec::with_capacity(capacity);
        self.start = 0;
        self.end = 0;
        self.len = 0;
    }


    #[inline(always)]
    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    #[inline(always)]
    pub fn is_full(&self) -> bool {
        self.len == self.capacity
    }

    #[inline(always)]
    fn inc_start(&mut self) {
        self.start += 1;
        self.start %= self.capacity;
    }

    #[inline(always)]
    fn inc_end(&mut self) {
        self.end += 1;
        self.end %= self.capacity;
    }

    pub fn push(&mut self, element: T) {
        if self.is_full() {
            return;
        }

        let cell = self.container.get_mut(self.start).unwrap();

        *cell = element;

        self.len += 1;
        self.inc_start();
        println!("push - start = {}, end = {}", self.start, self.end);
    }

    pub fn push_force(&mut self, element: T) {
        let cell = self.container.get_mut(self.start).unwrap();

        *cell = element;

        if self.is_full() {
            self.inc_end();
        } else {
            self.len += 1;
        }

        self.inc_start();
        println!("push force - start = {}, end = {}", self.start, self.end);
    }

    pub fn pop(&mut self) -> Option<T> {
        if !self.is_empty() {
            match self.buffer_type {
                BufferType::Queue => {
                    let cell = self.container.get_mut(self.end).unwrap();
                    let result = std::mem::take(cell);
                    self.len -= 1;
                    self.inc_end();
                    println!("queue pop - start = {}, end = {}", self.start, self.end);
                    println!("result = {}", result);
                    return Some(result);
                },
                BufferType::Stack => {
                    let mut last_idx = self.start;
                    if last_idx == 0 {
                        last_idx += self.capacity -1;
                    } else {
                        last_idx -= 1;
                    }
                    let cell = self.container.get_mut(last_idx).unwrap();
                    let result = std::mem::take(cell);
                    self.len -= 1;
                    self.start = last_idx;
                    println!("stack pop - start = {}, end = {}", self.start, self.end);
                    println!("result = {}", result);
                    return Some(result);
                },
            }
        } else {
            return None;
        }
    }

    pub fn peek_oldest(&self) -> Option<&T> {
        if !self.is_empty() {
            self.container.get(self.end)
        } else {
            None
        }
    }

    pub fn peek_newest(&self) -> Option<&T> {
        if !self.is_empty() {
            let index = (self.start + self.capacity - 1) % self.capacity;
            self.container.get(index)
        } else {
            None
        }
    }

    pub fn iter(&self) -> PushBufferIterator<T> {
        PushBufferIterator {
            ringvec: &self,
            current: self.end,
            length: self.len,
        }
    }
}

pub struct PushBufferIterator<'ring, T> {
    ringvec: &'ring PushBuffer<T>,
    current: usize,
    length: usize,
}

impl<'ring, T> Iterator for PushBufferIterator<'ring, T> {
    type Item = &'ring T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.length == 0 {
            None
        } else {
            let result = self.ringvec.container.get(self.current);

            self.length -= 1;
            self.current += 1;
            self.current %= self.ringvec.capacity;

            result
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        (self.length, Some(self.length))
    }
}

impl<'ring, T> ExactSizeIterator for PushBufferIterator<'ring, T> {}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn buffer_type_queue_pops_oldest() {
        let mut v = PushBuffer::new(BufferType::Queue, 3);

        assert!(v.is_empty());
        assert!(!v.is_full());

        v.push(1);
        v.push(2);
        v.push(3);
        v.push_force(4);
        v.push_force(5);

        assert!(!v.is_empty());
        assert!(v.is_full());

        assert_eq!(v.peek_oldest(), Some(&3));
        assert_eq!(v.peek_newest(), Some(&5));
        assert_eq!(v.pop(), Some(3));

        assert!(!v.is_empty());
        assert!(!v.is_full());

        assert_eq!(v.pop(), Some(4));
        assert_eq!(v.pop(), Some(5));
        assert_eq!(v.pop(), None);
        assert_eq!(v.pop(), None);
        assert_eq!(v.pop(), None);
        assert_eq!(v.pop(), None);

        assert!(v.is_empty());
        assert!(!v.is_full());
    }

    #[test]
    fn buffer_type_stack_pops_oldest() {
        let mut v = PushBuffer::new(BufferType::Stack, 3);

        assert!(v.is_empty());
        assert!(!v.is_full());

        v.push(1);
        v.push(2);
        v.push(3);
        v.push_force(4);
        v.push_force(5);

        assert!(!v.is_empty());
        assert!(v.is_full());

        assert_eq!(v.peek_oldest(), Some(&3));
        assert_eq!(v.peek_newest(), Some(&5));
        assert_eq!(v.pop(), Some(5));

        assert!(!v.is_empty());
        assert!(!v.is_full());

        assert_eq!(v.pop(), Some(4));
        assert_eq!(v.pop(), Some(3));
        assert_eq!(v.pop(), None);
        assert_eq!(v.pop(), None);
        assert_eq!(v.pop(), None);
        assert_eq!(v.pop(), None);

        assert!(v.is_empty());
        assert!(!v.is_full());
    }

    #[test]
    fn buffer_type_stack_pops_newest_when_force_push() {
        let mut v = PushBuffer::new(BufferType::Stack, 3);

        assert!(v.is_empty());
        assert!(!v.is_full());

        v.push_force(4);
        v.push_force(5);
        v.push_force(6);
        v.push_force(7);

        assert!(!v.is_empty());
        assert!(v.is_full());

        assert_eq!(v.peek_oldest(), Some(&5));
        assert_eq!(v.peek_newest(), Some(&7));
        assert_eq!(v.pop(), Some(7));

        assert!(!v.is_empty());
        assert!(!v.is_full());

        assert_eq!(v.pop(), Some(6));
        assert_eq!(v.pop(), Some(5));
        assert_eq!(v.pop(), None);
        assert_eq!(v.pop(), None);
        assert_eq!(v.pop(), None);
        assert_eq!(v.pop(), None);

        assert!(v.is_empty());
        assert!(!v.is_full());
    }

    #[test]
    fn buffer_iterates_returns_all_elements() {
        let mut v = PushBuffer::new(BufferType::Stack,3);

        assert!(v.is_empty());
        assert!(!v.is_full());

        v.push(1);
        v.push(2);
        v.push(3);

        dbg!(&v);

        let mut i = v.iter();

        assert_eq!(i.next(), Some(&1));
        assert_eq!(i.next(), Some(&2));
        assert_eq!(i.next(), Some(&3));
        assert_eq!(i.next(), None);
        assert_eq!(i.next(), None);
    }
}
