use std::marker::PhantomData;
use std::ptr::NonNull;

#[derive(Debug)]
pub struct Entry<T> {
    pub value: T,
    next: Option<NonNull<Entry<T>>>,
    prev: Option<NonNull<Entry<T>>>,
}

impl<T> Entry<T> {
    pub fn new(v: T) -> Self {
        Self {
            value: v,
            next: None,
            prev: None,
        }
    }
}

impl<T> Drop for Entry<T> {
    fn drop(&mut self) {
        println!("drop Entry")
    }
}

pub struct LinkedList<T> {
    head: Option<NonNull<Entry<T>>>,
    last: Option<NonNull<Entry<T>>>,
}

impl<T> Drop for LinkedList<T> {
    fn drop(&mut self) {
        println!("drop LinkedList");
        loop {
            if self.is_empty() {
                break;
            }
            let _ = self.pop_back();
        }
    }
}

impl<T> LinkedList<T> {
    pub fn new() -> Self {
        Self {
            head: None,
            last: None,
        }
    }

    pub fn push_back(&mut self, v: T) {
        let new_node = {
            unsafe { NonNull::new_unchecked(Box::leak(Box::new(Entry::new(v)))) }
        };
        dbg!(new_node);
        if self.is_empty() {
            self.head = Some(new_node);
            self.last = self.head;
        } else {
            unsafe {
                let current_last = self.last.as_mut().unwrap();
                (*current_last.as_ptr()).next = Some(new_node);
                (*new_node.as_ptr()).prev = Some(*current_last);
            }
            self.last = Some(new_node);
        }
        dbg!(self.last);
    }

    pub fn is_empty(&self) -> bool {
        self.head.is_none()
    }

    pub fn pop_back(&mut self) -> Option<Entry<T>> {
        if self.is_empty() {
            return None;
        }
        unsafe {
            let old_last = self.last.unwrap();
            let prev = (*old_last.as_ptr()).prev;
            if prev.is_none() {
                self.head = None;
                self.last = None;
            } else {
                self.last = prev;
            }
            let v = std::ptr::read(old_last.as_ptr());
            Some(v)
        }
    }

    pub fn pop_front(&mut self) -> Option<Entry<T>> {
        if self.is_empty() {
            return None;
        }
        unsafe {
            let mut head_ptr = self.head.unwrap();
            let new_head = (head_ptr.as_mut()).next;
            self.head = new_head;
            if self.head == self.last {
                self.last = None
            }
            let v = std::ptr::read(head_ptr.as_ptr());
            Some(v)
        }
    }

    pub fn iter(&self) -> Iter<'_, T> {
        Iter {
            pos: self.head,
            _marker: Default::default(),
        }
    }
}

pub struct Iter<'a, T: 'a> {
    pos: Option<NonNull<Entry<T>>>,
    _marker: PhantomData<&'a T>,
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        match self.pos {
            None => { None }
            Some(pos) if pos.as_ptr().is_null() => { None }
            Some(pos) => {
                unsafe {
                    let v = &(*pos.as_ptr()).value;
                    self.pos = (*pos.as_ptr()).next;
                    Some(v)
                }
            }
        }
    }
}


#[cfg(test)]
mod test {
    use super::LinkedList;

    #[test]
    fn test_push_pop() {
        let mut ll = LinkedList::new();
        ll.push_back(1);
        ll.push_back(2);
        assert_eq!(ll.pop_front().unwrap().value, 1);
        assert_eq!(ll.pop_front().unwrap().value, 2);
        ll.push_back(5);
        ll.push_back(6);
        assert_eq!(ll.pop_back().unwrap().value, 6);
        assert_eq!(ll.pop_back().unwrap().value, 5);
        assert!(ll.pop_back().is_none());
    }

    #[test]
    fn test_iter() {
        let mut ll = LinkedList::new();
        for i in 1..10 {
            ll.push_back(i)
        }
        for it in ll.iter() {
            println!("ele:{}", it)
        }
    }

    #[test]
    fn test_drop() {
        let mut ll = LinkedList::new();
        ll.push_back(100);
        ll.push_back(1000);
    }
}