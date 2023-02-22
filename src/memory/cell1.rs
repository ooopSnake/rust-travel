use std::cell::Cell;
use std::mem;
use std::rc::Rc;

///
/// std::cell::Cell 的set方法:先对旧值replace,然后再 drop 引起了我都兴趣
/// <https://github.com/rust-lang/rfcs/pull/1651#issuecomment-226927741>
///
/// stackoverflow 也有人提及
/// <https://stackoverflow.com/questions/74123700/why-does-set-method-defined-on-cellt-explicitly-drops-the-old-value-rust>
///
/// 老版本的rust对set实现如下:
/// ```rust
///    pub fn set(&self, value: T) {
///         unsafe {
///             *self.value.get() = value; // 这里会先原地析构旧值,再将新值移动到旧值的空间
///         }
///     }
/// ```
///
/// 有人在issue中构建了一个循环引用结构体,用来触发这个bug
///
/// 注意:由于实现改变了,所以当前版本下cell没有该问题
///
struct Evil(Box<u32>, Rc<Cell<Option<Evil>>>);

impl Drop for Evil {
    fn drop(&mut self) {
        mem::drop(self.1.take());  // Mess with the "other" node, which might be `self`.
        println!("{}", self.0);
        self.0.clone();  // use after free!
        println!("drop!");
    }
}

#[test]
pub fn test_evil_cell() {
    let mut x: Option<String> = Some("hey".to_owned());
    assert_eq!(x.as_deref_mut().map(|x| {
        x.make_ascii_uppercase();
        x
    }), Some("HEY".to_owned().as_mut_str()));

    let mut a = Rc::new(Cell::new(None));
    println!("1 strong:{},weak:{}", Rc::strong_count(&a), Rc::weak_count(&a));

    a.replace(Some(Evil(Box::new(5),
                        a.clone())));  // Make a reference cycle.

    println!("2 strong:{},weak:{}", Rc::strong_count(&a), Rc::weak_count(&a));
    // a.set(None);  // Trigger Evil::drop while in the cell (新版本不会触发)

    // 由于当前版本rust已经没有该问题,所以这里手动模拟之前的赋值操作,能够成功复现double free
    unsafe {
        let v = &mut &*Rc::as_ptr(&mut a);
        println!("3 strong:{},weak:{}", Rc::strong_count(&a), Rc::weak_count(&a));
        *v.as_ptr() = None;
    }
    println!("xxxxx")
}