use std::cell;
use std::cell::UnsafeCell;

struct Goods(i32);

struct CustomCell<T> {
    container: UnsafeCell<T>
}

impl<T> CustomCell<T> {
    fn new(o: T) -> CustomCell<T> {
        CustomCell {
            container: UnsafeCell::new(o)
        }
    }

    #[inline]
    fn get_mut(&mut self) -> &mut T {
        unsafe { &mut *self.container.get() }
    }
}

///
/// rust 很聪明
/// 下面的例子说明,多次可变借用时,
/// rust 编译器知道a1,a2两个可变借用
/// 不可能同时存在
///
/// 当a2产生后，a1没有被进行过任何读写操作
/// 所以a1已经expired了
#[test]
fn test_mut_borrow() {
    //let a1 = &mut x;
    let mut kk = CustomCell::new(Goods(0));
    //*a1 = 100;
    let a1 = kk.get_mut();
    (*a1).0 = 300;
    let a2 = kk.get_mut();
    (*a2).0 = 200;
}

#[test]
fn test_cell() {
    let mut c = cell::Cell::new(Goods(0));
    let c1 = c.get_mut();
    (*c1).0 = 200;
    let c2 = c.get_mut();
    (*c2).0 = 200;
}