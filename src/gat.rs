trait LendingIterator {
    type Item<'a> where Self: 'a;

    fn next<'a>(&'a mut self) -> Option<Self::Item<'a>>;
}

struct WindowsMut<'a, T> {
    slice: &'a mut [T],
    start: usize,
    window_size: usize,
}

impl<'t, T> LendingIterator for WindowsMut<'t, T> {
    type Item<'a> = &'a mut [T] where Self: 'a;

    fn next<'a>(&'a mut self) -> Option<Self::Item<'a>> {
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
