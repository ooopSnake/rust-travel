#![allow(unused, unused_variables)]

use std::cell::Cell;

struct NoCopyFoo {
    x: usize,
    y: usize,
}

///
/// Box
/// 1. 会被编译器优化为指针
/// 2. 如Cell不同的是,Cell可以在自身为不可变借用时,修改指针内部内容,而Box不可以
/// 3. Box是Unique的new type struct类型
/// 4. Unique具备rustc_layout_scalar_valid_range_start(1),其大小等于
///     其第一个属性 pointer: *const T
/// 5. Unique 被标记为 [repr(transparent)]
/// 6. 第4点+第5点,是Box能被优化为一个指针的原理
///
#[test]
fn test_box() {
    // new 参数的所有权被转移到Box中
    let mut b = Box::new(11);
    *b = 222;
    *b = 333;
    assert_eq!(*b, 333);
    // 如果是cell,就不需要绑定为mut
    // 同样 new 参数的所有权被转义到Cell中
    // 注意,Cell<T>的T必须能够被copy
    let cell = Cell::new(233);
    cell.set(555);
    assert_eq!(cell.get(), 555);

    //
    let mut cell2 = Cell::new(NoCopyFoo { x: 1, y: 2 });
    // 由于 impl<T: Copy> Cell<T>
    // 而 NoCopyFoo 没有实现Copy
    // 导致无法拷贝,所以Cell.get方法不可用
    // 此时Cell只能移动,不能拷贝
    let c3 = &*cell2.get_mut();
}