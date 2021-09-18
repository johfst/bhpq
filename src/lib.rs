#![allow(dead_code)]
use std::collections::LinkedList;

#[derive(Debug)]
pub struct BHPQ<T> {
    top: Option<usize>,
    max_priority: usize,
    len: u32,
    array: Vec<LinkedList<T>>,
}

pub impl<T> BHPQ<T> {
    pub fn new(max_priority: usize) -> BHPQ<T> {
        let mut bhpq = BHPQ::<T> {
            top: None,
            max_priority,
            len: 0,
            array: Vec::<LinkedList<T>>::with_capacity(max_priority-1),
        };
        for _ in 0..max_priority {
            bhpq.array.push(LinkedList::<T>::new());
        }
        //bhpq.array.resize_with(size, LinkedList::<T>);
        return bhpq;
    }

    pub fn push(&mut self, priority: usize, value: T) -> Result<(), PriorityError> {
        //if priority > self.max_priority
        self.get_priority_mut(priority)?.push_front(value);
        self.len += 1;

        match self.top {
            None => self.top = Some(priority),
            Some(k) => self.top = Some(std::cmp::min(k, priority)),
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
        for k in start..self.max_priority {
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
        if priority > self.max_priority { 
            Err(PriorityError::new(priority))
        } else { Ok(&mut self.array[priority]) }
    }

    pub fn get_priority(&self, priority: usize) -> Result<&LinkedList<T>, PriorityError> {
        if priority > self.max_priority { 
            Err(PriorityError::new(priority))
        } else { Ok(&self.array[priority]) }
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


/*
fn main() -> Result<(), PriorityError> {
    let mut bhpq = BHPQ::<String>::new(2);
    bhpq.push(3, String::from("c"))?;
    bhpq.push(4, String::from("d"))?;
    bhpq.push(1, String::from("a"))?;
    bhpq.push(3, String::from("cat"))?;
    println!("bhpq: {:?}", bhpq);
    println!("empty? {}", bhpq.is_empty());
    if let Some(val) = bhpq.pop() {
        println!("pop: {}", val);
    }
    println!("bhpq: {:?}", bhpq);
    bhpq.clear();
    Ok(())
}
*/
