use std::rc::Rc;

#[test]
fn test_rc() {
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