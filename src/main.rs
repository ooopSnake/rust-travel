#![feature(fn_traits)]

fn call<T, R>(f: T) -> R where T: FnOnce() -> R {
    f()
}

fn main() {
    let mut x = 0;
    let incr_x = move || x += 1;
    call(incr_x);
    call(incr_x);
}