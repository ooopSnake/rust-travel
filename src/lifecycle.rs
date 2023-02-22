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

///
/// # 生命周期过长的演示
///
mod problem_lifetime_toolong {
    struct Interface<'a> {
        manager: &'a mut Manager<'a>,
    }

    impl Interface<'_> {
        pub fn noop(self) {
            println!("interface consumed");
        }
    }

    struct Manager<'a> {
        text: &'a str,
    }

    struct List<'a> {
        manager: Manager<'a>,
    }

    impl<'a, 'b: 'a, 'c: 'a> List<'b> {
        // 此处理解为:
        // 创建一个对List的可变借用,其生命周期与List实例相同
        pub fn get_interface(&'c mut self) -> Interface<'a> {
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
