use std::cell::UnsafeCell;

///
/// ### UnsafeCell 理解
///
/// 1. unsafeCell 被标记为 lang=unsafe_cell,其被编译器特殊照顾
/// 2. repr=transparent表示其是一个透明层,内存布局与T一致
///     且其本身作为一个new type struct.相当于对T实现了相关扩展.
///
struct CellV2<T> {
    value: UnsafeCell<T>
}

impl<T> CellV2<T> {
    fn new(v: T) -> Self where T: Copy {
        CellV2 { value: UnsafeCell::new(v) }
    }

    fn set(&self, v: T) where T: Copy {
        unsafe { *self.value.get() = v }
    }

    fn get(&self) -> T where T: Copy {
        unsafe { *self.value.get() }
    }
}

struct Table<'arg> {
    cell: CellV2<&'arg isize>
}

fn evil<'long>(t: &Table<'long>, s: &'long isize)

{
    // The following assignment is not legal,
    // but it escapes from lifetime checking
    let u: &Table<'long> = t;
    u.cell.set(s);
}

fn innocent(t: &Table) {
    let foo: isize = 1;
    // 得益于 UnsafeCell , 编译器可以确定的发现foo的生命周期不足
    // 编译失败
    evil(t, &foo);
}

#[test]
fn test_x() {
    let local = 100;
    let table = Table { cell: CellV2::new(&local) };
    innocent(&table);
    // reads `foo`, which has been destroyed
    let p = table.cell.get();
    println!("{}", p);
}