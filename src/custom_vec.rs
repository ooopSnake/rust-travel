//! 自定义Vec
//! 知识点: ptr::drop_in_place, alloc::alloc/dealloc/Layout , NonNull
//! ## Custom Vector in Rust
//!
//! |    Method    |              Doc               |
//! |:------------:|:------------------------------:|
//! |     push     |         append to tail         |
//! |     pop      | remove and return last element |
//! |     cap      |               -                |
//! |     len      |               -                |
//! | IntoIterator |        support iterator        |
//!
//! ```rust
//! fn main() {
//!     let mut arr = container::vec::Vector::new();
//!     arr.push(0);
//!     arr.push(1);
//!     arr.push(2);
//!     for it in &arr {
//!         println!("- {}", it);
//!     }
//!     assert_eq!(*arr.get_ref(0), 0);
//!     assert_eq!(*arr.get_ref(1), 1);
//!     assert_eq!(*arr.get_ref(2), 2);
//!     assert_eq!(arr.pop().unwrap(), 2);
//!     assert_eq!(arr.pop().unwrap(), 1);
//!     assert_eq!(arr.pop().unwrap(), 0);
//! }
//! ```

use std::alloc::Layout;
use std::borrow::Borrow;
use std::marker::PhantomData;
use std::ops::Deref;
use std::ptr;
use std::ptr::NonNull;

pub struct Vector<T> {
    ptr: NonNull<T>,
    len: usize,
    cap: usize,
}

impl<T> Drop for Vector<T> {
    fn drop(&mut self) {
        unsafe {
            std::ptr::drop_in_place(
                std::ptr::slice_from_raw_parts_mut(self.ptr.as_ptr(), self.len));
            std::alloc::dealloc(self.ptr.as_ptr() as *mut u8,
                                Layout::array::<T>(self.cap).unwrap())
        }
    }
}

pub struct VectorIter<'a, T> {
    begin: NonNull<T>,
    end: NonNull<T>,
    marker: PhantomData<&'a T>,
}

impl<'a, T> std::iter::IntoIterator for &'a Vector<T> {
    type Item = &'a T;
    type IntoIter = VectorIter<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        unsafe {
            VectorIter {
                begin: NonNull::new_unchecked(self.ptr.as_ptr()),
                end: NonNull::new_unchecked(self.ptr.as_ptr().add(self.len)),
                marker: PhantomData,
            }
        }
    }
}

impl<T> Deref for Vector<T> {
    type Target = [T];

    fn deref(&self) -> &Self::Target {
        unsafe {
            &*ptr::slice_from_raw_parts(self.ptr.as_ptr(), self.len)
        }
    }
}

impl<'a, T> Iterator for VectorIter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.begin == self.end {
            None
        } else {
            unsafe {
                let out = Some(&*self.begin.as_ptr());
                self.begin = NonNull::new_unchecked(self.begin.as_ptr().add(1));
                out
            }
        }
    }
}

impl<T> Vector<T> {
    pub fn new() -> Vector<T> {
        Vector {
            ptr: NonNull::dangling(),
            len: 0,
            cap: 0,
        }
    }

    fn len(&self) -> usize {
        self.len
    }

    fn cap(&self) -> usize {
        self.cap
    }

    fn check_grow(&mut self) {
        unsafe {
            if self.cap <= 0 {
                let layout = std::alloc::Layout::array::<T>(4).unwrap();
                let arr_mem = std::alloc::alloc(layout) as *mut T;
                self.ptr = NonNull::new_unchecked(arr_mem);
                self.cap = 4;
            } else if self.cap == self.len {
                let layout = std::alloc::Layout::array::<T>(self.cap as usize).unwrap();
                let new_cap = self.cap * 2;
                let new_mem = std::alloc::realloc(self.ptr.as_ptr() as *mut u8,
                                                  layout, new_cap) as *mut T;
                self.ptr = NonNull::new_unchecked(new_mem);
                self.cap = new_cap;
            }
        }
    }

    /// safety: Vector not impl Send or Syn
    pub fn get_ref(&self, idx: usize) -> &T {
        unsafe {
            &*self.ptr.as_ptr().add(idx)
        }
    }

    pub fn get_mut_ref(&mut self, idx: usize) -> &mut T {
        unsafe {
            &mut *self.ptr.as_ptr().add(idx)
        }
    }

    pub fn push(&mut self, ele: T) {
        self.check_grow();
        unsafe {
            self.ptr.as_ptr().add(self.len).write(ele);
            self.len += 1
        }
    }


    pub fn pop(&mut self) -> Option<T> {
        if self.len == 0 {
            return None;
        }
        unsafe {
            let r = self.ptr.as_ptr().add(self.len - 1).read();
            self.len -= 1;
            Some(r)
        }
    }
}


#[test]
fn test_vec() {
    let mut arr = Vector::new();
    arr.push(0);
    arr.push(1);
    arr.push(2);
    for it in &arr {
        println!("- {}", it);
    }
    assert_eq!(*arr.get_ref(0), 0);
    assert_eq!(*arr.get_ref(1), 1);
    assert_eq!(*arr.get_ref(2), 2);
    assert_eq!(arr.pop().unwrap(), 2);
    assert_eq!(arr.pop().unwrap(), 1);
    assert_eq!(arr.pop().unwrap(), 0);
}

