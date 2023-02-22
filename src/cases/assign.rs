///
/// rust 直接赋值的解糖之后的语句
///
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

fn test_assign() -> Data {
    let mut x = Data { name: "test_assign.x", x: 1, y: Default::default() };
    let mut y = Data { name: "test_assign.y", x: 1, y: Default::default() };
    x = y; // 直接赋值
    return x;
}

fn test_ptr_assign() -> Data {
    let mut x = Data { name: "test_ptr_assign.x", x: 1, y: Default::default() };
    let y = Data { name: "test_ptr_assign.y", x: 1, y: Default::default() };
    // 直接赋值等于
    unsafe {
        std::ptr::drop_in_place(&mut x);
        std::ptr::write(&mut x, y);
    }
    return x;
}

fn example() {
    let a = test_assign();
    let b = test_ptr_assign();
}
