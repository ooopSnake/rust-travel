#![allow(unused)]

fn wtf() -> u8 {
    1
}


///
/// 函数遮蔽
/// 1.在同一个作用域中不能定义多个同名函数
/// 2.默认的函数定义只在当前作用域内有效,会屏蔽作用域外的同名函数
///
#[test]
fn test_fn_shadow() {
    assert_eq!(wtf(), 2);
    {
        assert_eq!(wtf(), ());
        fn wtf() {
            println!("inner")
        }
    }
    fn wtf() -> u8 {
        2
    }
}