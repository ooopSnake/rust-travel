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
fn test_lifecycle() {
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

