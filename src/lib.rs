#![allow(dead_code)]
use std::collections::LinkedList;
use std::error::Error;
use std::cmp::min;
use std::fmt;

#[derive(Debug)]
pub struct BHPQ<T> {
    top: Option<usize>,
    priority_slots: usize,
    len: u32,
    array: Vec<LinkedList<T>>,
}

impl<T> BHPQ<T> {
    pub fn new(priority_slots: usize) -> BHPQ<T> {
        let mut bhpq = BHPQ::<T> {
            top: None,
            priority_slots,
            len: 0,
            array: Vec::<LinkedList<T>>::with_capacity(priority_slots),
        };
        for _ in 0..priority_slots {
            bhpq.array.push(LinkedList::<T>::new());
        }
        //bhpq.array.resize_with(size, LinkedList::<T>);
        return bhpq;
    }

    pub fn push(&mut self, priority: usize, value: T) -> Result<(), PriorityError> {
        //if priority >= self.priority_slots, error
        self.get_priority_mut(priority)?.push_front(value);
        self.len += 1;

        match self.top {
            None => self.top = Some(priority),
            Some(k) => self.top = Some(min(k, priority)),
        };
        Ok(())
    }

    pub fn pop(&mut self) -> Option<T> {
        match self.top {
            None => None,
            Some(k) => {
                let value = self.array[k].pop_front();
                self.top = self.search_top(k);
                self.len -= 1;
                value
            },
        }
    }

    fn search_top(&self, start: usize) -> Option<usize> {
        for k in start..self.priority_slots {
            if !self.array[k].is_empty() { return Some(k) };
        }
        return None;
    }

    pub fn peek(&self) -> Option<&T> {
        match self.top {
            None => None,
            Some(k) => self.array[k].front(),
        }
    }
    
    pub fn peek_mut(&mut self) -> Option<&mut T> {
        match self.top {
            None => None,
            Some(k) => self.array[k].front_mut(),
        }
    }

    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    pub fn clear(&mut self) {
        self.len = 0;
        loop {
            if let Some(top) = self.top {
                self.array[top].clear();
                self.top = self.search_top(top+1);
            } else { break; }
        }
    }

    pub fn get_priority_mut(&mut self, priority: usize) 
        -> Result<&mut LinkedList<T>, PriorityError> {
        if priority >= self.priority_slots { 
            Err(PriorityError::new(priority))
        } else { Ok(&mut self.array[priority]) }
    }

    pub fn get_priority(&self, priority: usize) -> Result<&LinkedList<T>, PriorityError> {
        if priority >= self.priority_slots { 
            Err(PriorityError::new(priority))
        } else { Ok(&self.array[priority]) }
    }
}

impl<T> Iterator for BHPQ<T> {
    type Item = T;

    fn next(&mut self) -> Option<T> {
        self.pop()
    }
}

#[derive(Debug, Copy, Clone, Hash)]
pub struct PriorityError {
    priority: usize,
}

impl PriorityError {
    fn new(priority: usize) -> PriorityError {
        PriorityError { priority, }
    }
}

impl fmt::Display for PriorityError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "tried to push priority {} larger than upper bound", self.priority)
    }
}

impl Error for PriorityError {}

#[cfg(test)]
mod tests {
    use crate::BHPQ;

    #[test]
    fn new() {
        let _bhpq: BHPQ<i32> = BHPQ::new(5);
    }

    #[test]
    fn push() {
        let mut bhpq: BHPQ<i32> = BHPQ::new(5);
        bhpq.push(2, 25).unwrap();
        assert!(bhpq.array[2].contains(&25));
    }

    #[test]
    #[should_panic]
    fn push_priorityerror() {
        let mut bhpq: BHPQ<i32> = BHPQ::new(2);
        bhpq.push(6, 65).unwrap();
    }

    #[test]
    fn pop_value() {
        let mut bhpq: BHPQ<i32> = BHPQ::new(2);
        bhpq.push(1, 15).unwrap();
        assert_eq!(bhpq.pop().unwrap(), 15);
        assert!(bhpq.is_empty());
    }

    #[test]
    #[should_panic]
    fn pop_none() {
        let mut bhpq: BHPQ<i32> = BHPQ::new(2);
        bhpq.pop().unwrap();
        bhpq.push(1, 15).unwrap();
        bhpq.pop().unwrap();
        bhpq.pop().unwrap();
    }

    #[test]
    fn pop_order() {
        let mut bhpq: BHPQ<i32> = BHPQ::new(3);
        bhpq.push(2, 25).unwrap();
        bhpq.push(1, 15).unwrap();
        bhpq.push(0, 5).unwrap();
        assert_eq!(bhpq.pop().unwrap(), 5);
        assert_eq!(bhpq.pop().unwrap(), 15);
        assert_eq!(bhpq.pop().unwrap(), 25);
    }

    #[test]
    fn peek() {
        let mut bhpq: BHPQ<i32> = BHPQ::new(2);
        bhpq.push(1, 15).unwrap();
        assert_eq!(*bhpq.peek().unwrap(), 15);
    }

    #[test]
    fn peek_mut() {
        let mut bhpq: BHPQ<i32> = BHPQ::new(2);
        bhpq.push(1, 15).unwrap();
        let top: &mut i32 = bhpq.peek_mut().unwrap();
        *top = 16;
        assert_eq!(*bhpq.peek().unwrap(), 16);
    }

    #[test]
    fn is_empty() {
        let bhpq: BHPQ<i32> = BHPQ::new(2);
        assert!(bhpq.is_empty());
    }

    #[test]
    fn clear() {
        let mut bhpq: BHPQ<i32> = BHPQ::new(2);
        bhpq.push(1, 15).unwrap();
        bhpq.clear();
        assert!(bhpq.is_empty());
    }

    #[test]
    fn get_priority() {
        let mut bhpq: BHPQ<i32> = BHPQ::new(2);
        bhpq.push(1, 15).unwrap();
        bhpq.get_priority(1).unwrap();
    }

    #[test]
    fn get_priority_mut() {
        use std::collections::LinkedList;
        let mut bhpq: BHPQ<i32> = BHPQ::new(2);
        bhpq.push(1, 15).unwrap();
        let ll: &mut LinkedList<i32> = bhpq.get_priority_mut(1).unwrap();
        let front = ll.front_mut().unwrap();
        *front = 16;
        assert_eq!(*bhpq.peek().unwrap(), 16);
    }
}
