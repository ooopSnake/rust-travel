//! 知识点: `lifecycle annotation`, `generic with lifecycle`,
//! [Higher-ranked trait bounds](https://doc.rust-lang.org/reference/trait-bounds.html#higher-ranked-trait-bounds)

struct Parser<'a, 'b: 'a> {
    context: &'a Option<&'b str>,
}

impl<'a, 'b: 'a> Parser<'a, 'b> {
    fn parse(&self) -> &'a Option<&'b str> {
        self.context
    }
}

fn parse_context<'a, 'b: 'a>(context: &'a Option<&'b str>) -> Result<(), &'b str> {
    let r = Parser { context: &context }.parse();
    if let &Some(v) = r {
        return Err(v);
    }
    Err("xxxxx")
}


struct Foo<'a, T: ?Sized + 'a> {
    s: &'a T,
}

fn box_foo<T: ?Sized>(s: &T) -> Box<Foo<T>> where for<'a> T: 'a {
    Box::new(Foo { s })
}


#[test]
fn test() {
    let v = Some("aabbccdd");
    if let Err(x) = parse_context(&v) {
        println!("{}", x)
    }
    let vvv = vec![1u32, 2, 3, 4];
    {
        let bx = box_foo(&vvv);
        println!("1:{:?}", &bx.s);
    }
    println!("2:{:?}", &vvv);
}

/// # 经典生命周期问题1
/// 故障代码
/// ```rust
/// struct Interface<'a> {
///     manager: &'a mut Manager<'a>
/// }
///
/// impl<'a> Interface<'a> {
///     pub fn noop(self) {
///         println!("interface consumed");
///     }
/// }
///
/// struct Manager<'a> {
///     text: &'a str
/// }
///
/// struct List<'a> {
///     manager: Manager<'a>,
/// }
///
/// impl<'a> List<'a> {
///     // 注意此处
///     pub fn get_interface(&'a mut self) -> Interface {
///         Interface {
///             manager: &mut self.manager
///         }
///     }
/// }
///
/// fn main() {
///     let mut list = List {
///         manager: Manager {
///             text: "hello"
///         }
///     };
///
///     list.get_interface().noop();
///
///     println!("Interface should be dropped here and the borrow released");
///
///     // this fails because inmutable/mutable borrow
///     // but Interface should be already dropped here and the borrow released
///     use_list(&list);
/// }
///
/// fn use_list(list: &List) {
///     println!("{}", list.manager.text);
/// }
/// ```
///
/// 编译结果
///
/// ```bash
///    Compiling playground v0.0.1 (/playground)
/// error[E0502]: cannot borrow `list` as immutable because it is also borrowed as mutable
///   --> src/main.rs:40:14
///    |
/// 34 |     list.get_interface().noop();
///    |     -------------------- mutable borrow occurs here
/// ...
/// 40 |     use_list(&list);
///    |              ^^^^^
///    |              |
///    |              immutable borrow occurs here
///    |              mutable borrow later used here
///
/// For more information about this error, try `rustc --explain E0502`.
/// error: could not compile `playground` due to previous error
/// ```
pub mod problem_lifetime_toolong_1 {
    struct Manager<'a> {
        text: &'a str,
    }

    struct Interface<'a, 'b: 'a> {
        manager: &'a mut Manager<'b>,
    }

    impl Interface<'_, '_> {
        pub fn noop(self) {
            println!("interface consumed");
        }
    }

    struct List<'a> {
        manager: Manager<'a>,
    }


    impl<'b> List<'b> {
        /// 此处理解为:
        /// 创建一个对List的可变借用,其生命周期与List实例相同
        pub fn get_interface<'c>(&'c mut self) -> Interface<'c, 'b> {
            Interface {
                manager: &mut self.manager
            }
        }
    }


    fn use_list(list: &List) {
        println!("{}", list.manager.text);
    }

    #[test]
    fn test_lifetime_too_long() {
        let mut list = List {
            manager: Manager {
                text: "hello"
            }
        };
        list.get_interface().noop();

        println!("Interface should be dropped here and the borrow released");

        // this fails because inmutable/mutable borrow
        // but Interface should be already dropped here and the borrow released
        use_list(&list);
    }
}

///
///
/// # 经典生命周期问题2
/// 故障代码:
/// ```rust
/// fn bar(writer: &mut Writer) {
///     baz(&mut writer.indent());
///     writer.write("world");
/// }
///
/// fn baz(writer: &mut Writer) {
///     writer.write("hello");
/// }
///
/// pub struct Writer<'a> {
///     target: &'a mut String,
///     indent: usize,
/// }
///
/// impl<'a> Writer<'a> {
///     // 凡是出现将对象生命周期标到`&self`或者`&mut self`上的
///     // 99.9%的情况都是错误. 剩下0.1%可能是故意设计.
///     // 所以直接判定为`错误用法`即可
///     fn indent(&'a mut self) -> Writer<'a> {
///         Writer {
///             target: self.target,
///             indent: self.indent + 1,
///         }
///     }
///
///     fn write(&mut self, s: &str) {
///         for _ in 0..self.indent {
///             self.target.push(' ');
///         }
///         self.target.push_str(s);
///         self.target.push('\n');
///     }
/// }
/// ```
///
pub mod problem_lifetime_toolong_2 {
    fn bar(writer: &mut Writer) {
        baz(&mut writer.indent());
        writer.write("world");
    }

    fn baz(writer: &mut Writer) {
        writer.write("hello");
    }

    pub struct Writer<'a> {
        target: &'a mut String,
        indent: usize,
    }

    impl Writer<'_> {
        fn indent(&mut self) -> Writer {
            Writer {
                target: self.target,
                indent: self.indent + 1,
            }
        }

        fn write(&mut self, s: &str) {
            for _ in 0..self.indent {
                self.target.push(' ');
            }
            self.target.push_str(s);
            self.target.push('\n');
        }
    }
}