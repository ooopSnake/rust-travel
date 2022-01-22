#![allow(unused)]

//!
//! # Rust 闭包的理解
//! 好文推荐: <https://ioover.net/dev/rust-closure/>
//!
//! rust闭包就是捕获了当前上下文变量的结构体
//!
//! rust中,即使两个闭包参数和返回值一样,他们也**不可以**被视为同一种类型,
//! 因为他们背后的匿名结构体不同,有着不同的**大小**,**字段**,以及**生命周期**
//!
//! ## 规则
//!
//! ### 1. 最简单的情况: move
//!
//! 捕获规则最简单的情况是 ```move || {...}```,他会尝试获取closure中使用到的值的 *ownership*
//! 如果值时*Copy*的则copy一个
//!
//! ### 2. 默认的捕获方式:
//!
//! 1. 如果可以,则尽量用*&*借用
//! 2. 否则,如果可以,则总是&mut借用
//! 3. 最后,无计可施必须要ownership的话,才会move
//!
//! 捕获之后,根据closure中**如何使用捕获到的值**,
//! 编译器会为closure自动实现**函数traits**
//! 实现了那些traits与捕获方式或者捕获了那些变量是无关的
//!
//! **实现了那些traits,只跟这些变量在closure中的使用方式有关!**
//!
//! ### Fn Trait实现规则:
//! - 所有函数都至少能调用一次,则实现`FnOnce`
//! - 对于不会从closure结构体中转移的变量,实现`FnMut`
//! - 对于不会修改匿名结构体中变量的closure实现`Fn`

struct Foo(usize, usize);

impl Foo {
    fn wtf_mut(&mut self) -> usize {
        println!("hello world");
        self.0 += 100;
        self.1 += 100;
        self.0 + self.1
    }

    fn wtf_once(self) -> usize {
        self.0 + self.1
    }

    fn wtf_fn(&self) -> usize {
        self.0 + self.1
    }
}

#[test]
fn test_fn_once() {
    let k = Foo(100, 200);
    let g = || k.wtf_once();
    println!("call 1:{}", g());
}

#[test]
fn test_fn_mut() {
    let mut k = Foo(100, 200);
    let mut g = || {
        let x = &mut k;
        x.wtf_mut()
    };
    println!("call 1:{}", g());
    println!("call 2:{}", g());
    println!("call 3:{}", g());
}

#[test]
fn test_fn() {
    let k = Foo(10, 20);
    let g = || k.wtf_fn();
    println!("call 1:{}", g());
    println!("call 2:{}", g());
    println!("call 3:{}", g());
}