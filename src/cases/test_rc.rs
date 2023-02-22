#![allow(unused)]

use std::cell::{RefCell, RefMut};
use std::collections::HashMap;
use std::mem;
use std::rc::Rc;

#[test]
fn test_rc() {
    let mut ggg = mem::ManuallyDrop::new(Box::new(111));
    // explicit drop
    unsafe { mem::ManuallyDrop::drop(&mut ggg); }
    let r = Rc::new(123);
    let _x = r.clone();
    let _y = r.clone();
    println!("{:?}", Rc::strong_count(&r));
    let _z = Rc::downgrade(&r);
    println!("{:?}", Rc::weak_count(&r));
    let _k = &*r;
    println!("{:?}", Rc::strong_count(&r));
    let g = *r;
    println!("{},{}", g, *r)
}

#[derive(Debug)]
enum IntOrFloat {
    Int(i32),
    Float(f32),
}

trait IntOrFloatTrait {
    fn to_int_or_float(&self) -> IntOrFloat;
}

impl IntOrFloatTrait for i32 {
    fn to_int_or_float(&self) -> IntOrFloat {
        IntOrFloat::Int(*self)
    }
}

impl IntOrFloatTrait for f32 {
    fn to_int_or_float(&self) -> IntOrFloat {
        IntOrFloat::Float(*self)
    }
}

#[test]
fn test_rc_cell() {
    let g = IntOrFloatTrait::to_int_or_float(&100i32);
    if let IntOrFloat::Int(v) = g {
        assert_eq!(v, 100);
    }

    let shared_map: Rc<RefCell<_>> =
        Rc::new(RefCell::new(HashMap::new()));
    // Create a new block to limit the scope of the dynamic borrow
    {
        let mut map: RefMut<_> = shared_map.borrow_mut();
        map.insert("africa", 92388);
        map.insert("kyoto", 11837);
        map.insert("piccadilly", 11826);
        map.insert("marbles", 38);
    }

    // Note that if we had not let the previous borrow of the cache fall out
    // of scope then the subsequent borrow would cause a dynamic thread panic.
    // This is the major hazard of using `RefCell`.
    let total: i32 = shared_map.borrow().values().sum();
    println!("{}", total);
}

