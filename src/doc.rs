//!
//! 这是`doc`模块的文档,这条是单行注释.
//!

// --------    甚至可以分开写.... --------

/*!

甚至可以出现多行注释 :P

 */

/// 文档注释:单行注释
/// 也可以出现在内部模块中
mod mod_doc1 {
    #[macro_export]
    macro_rules! doc_foo {
      () => {}
    }

    /// 定义了一个结构体,用作示例.
    struct MyStruct {
        /// 字段也可以有文档
        x: i32,
        /// 字段也可以有文档
        y: i32,
    }

    ///
    /// 这是文档注释,可以`cargo doc`里能看到我.
    ///
    /// 可以链接函数 [查看函数:example_func2](fn@self::example_func2)
    ///
    /// 可以链接结构体 [查看结构体:MyStruct](struct@self::MyStruct)
    ///
    /// 可以链接到宏 [查看宏:doc_foo!](`doc_foo!`)
    ///
    pub fn example_func1() {
        // 这是普通代码注释,不会体现到文档中
        println!("example_func1")
    }

    // 这是普通注释,只有开发者才能看到,doc中不会体现
    pub fn example_func2() {}

    ///
    ///
    /// 文档里可以出现测试代码
    ///
    /// ```rust
    ///
    /// self::example_func2();
    ///
    /// ```
    ///
    /// `should_panic`用于指示改测试会造成panic,并且文档生成的格式也会有所变化,
    /// 你会看到一个红色的感叹标志.
    ///
    /// ```rust,should_panic
    /// # println!("这条打印,从测试中隐藏了,文档中不会出现")
    /// self::example_func3();
    ///
    /// ```
    ///
    pub fn example_func3() {
        panic!("boom!")
    }

    ///
    /// 文档别名,在`doc`中使用别名也能够搜索到
    /// ```
    /// #[doc(alias = "x")]
    /// #[doc(alias = "big")]
    /// pub struct BigX;
    ///
    /// #[doc(alias("y", "big"))]
    /// pub struct BigY;
    /// ```
    ///
    pub fn example_func4() {}
}

/**

文档注释:多行注释

可以有很多行

这是块注释

这是块注释

 */
mod mod_doc2 {}

