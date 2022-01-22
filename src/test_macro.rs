//! 宏 知识点: [macro by example](https://doc.rust-lang.org/reference/macros-by-example.html),
//! [macro quick book](https://www.bookstack.cn/read/DaseinPhaos-tlborm-chinese/README.md)

macro_rules! to_unit_ref {
    ($($x:tt)*) =>(&());
}

macro_rules! const_count {
    ($($i:expr),*) => ({
     const _V :usize = <[&()]>::len(&[$(to_unit_ref!($i)),*]);
     _V
    } );
}

#[test]
fn test_macro1() {
    const CNT: usize = const_count![1,2,3,4,5,6];
    println!("cnt={}", CNT);
}