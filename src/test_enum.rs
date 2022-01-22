#[derive(Debug, PartialEq)]
enum Wrapper<T> {
    Value(T),
    Empty,
}

impl<T> Wrapper<T> {
    fn clear(&mut self) {
        match self {
            Wrapper::Value(_) => {
                *self = Self::Empty
            }
            _ => {}
        }
    }
}

#[test]
fn test_enum() {
    let x = &1;
    let wref = &mut Wrapper::Value(x);
    if let Wrapper::Value(v) = wref {
        println!("vv:{}", *v)
    }
    wref.clear();
    assert_eq!(*wref, Wrapper::Empty);
}
