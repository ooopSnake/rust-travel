trait LendingIterator {
    type Item<'a> where Self: 'a;

    fn next(&mut self) -> Option<Self::Item<'_>>;
}

struct WindowsMut<'a, T> {
    slice: &'a mut [T],
    start: usize,
    window_size: usize,
}

impl<'t, T> LendingIterator for WindowsMut<'t, T> {
    type Item<'a> = &'a mut [T] where Self: 'a;

    fn next(&mut self) -> Option<Self::Item<'_>> {
        let retval = self.slice[self.start..].get_mut(..self.window_size)?;
        self.start += 1;
        Some(retval)
    }
}

#[test]
fn test() {
    let mut v: [i32; 8] = Default::default();
    let mut wm = WindowsMut {
        slice: &mut v,
        start: 0,
        window_size: 2,
    };
    let m = wm.next();
    m.map(|v| {
        for x in v {
            println!("{}", x)
        }
    });
    let m = wm.next();
}

/// [`引用自 rust lang blog`](https://blog.rust-lang.org/2021/08/03/GATs-stabilization-push.html)
pub mod gat_1 {
    use std::rc::Rc;
    use std::sync::Arc;

    trait PointerFamily {
        type PointerType<T>;
    }

    struct RcPointer;

    impl PointerFamily for RcPointer {
        type PointerType<T> = Rc<T>;
    }

    struct ArcPointer;

    impl PointerFamily for ArcPointer {
        type PointerType<T> = Arc<T>;
    }

    /// MyDataStructure 通过`PointerSel`能够选择data的数据实现.
    struct MyDataStructure<PointerSel: PointerFamily> {
        data: PointerSel::PointerType<String>,
    }
}