//!
//! rust 深入理解赋值语句背后的 *秘密*
//!

struct Data {
    name: &'static str,
    x: i32,
    y: [i64; 16],
}

impl Drop for Data {
    fn drop(&mut self) {
        println!("{} drop!", self.name);
    }
}



/// 直接赋值 x = y; 生成的汇编中:
/// 首先mem::drop_in_place(x); 再将y的值写入到x中
/// 等同于下面的这段代码:
/// ```
/// unsafe {
///         std::ptr::drop_in_place(&mut x);
///         // 另外 ptr::write 会执行小对象优化,如果对象过大,会直接调用 memcpy
///         std::ptr::write(&mut x, y);
/// }
fn test_assign() -> Data {
    let mut x = Data { name: "test_assign.x", x: 1, y: Default::default() };
    let mut y = Data { name: "test_assign.y", x: 1, y: Default::default() };
    x = y; // 直接赋值
    return x;
}

/// [`test_assign`](fn@test_assign) 的解糖展开
fn test_ptr_assign() -> Data {
    let mut x = Data { name: "test_ptr_assign.x", x: 1, y: Default::default() };
    let y = Data { name: "test_ptr_assign.y", x: 1, y: Default::default() };
    unsafe {
        std::ptr::drop_in_place(&mut x);
        std::ptr::write(&mut x, y);
    }
    return x;
}

#[test]
fn test() {
    let a = test_assign();
    let b = test_ptr_assign();
}
