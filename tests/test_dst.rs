use std::mem;
use std::ops::Deref;

// trait Beep {
//
// }
//
// impl<T> Beep for T where T: Bird {
//     fn beep(&self) {
//         self.fly();
//         println!("beep after fly");
//     }
// }


trait Bird {
    fn fly(&self);
    fn beep(&self) {
        println!("beep")
    }
}

struct Duck {
    v1: i32
}

struct Swan {
    v2: i64
}

impl Bird for Duck {
    fn fly(&self) {
        println!("duck duck");
    }

    fn beep(&self) {
        println!("duck beep")
    }
}

impl Bird for Swan {
    fn fly(&self) {
        println!("swan swan")
    }
}

/// 参数是 trait object 类型，p 是一个胖指针
///  `*const ()` 可以视为C/C++中的void*
fn print_trait_object(p: &dyn Bird) {

    // 使用transmute执行强制类型转换，把变量p的内部数据取出来
    let (data, vtable): (usize, usize) = unsafe { mem::transmute(p) };
    println!("TraitObject    [data:0x{:x}, vtable:0x{:x}]", data, vtable);
    unsafe {
        // 使用as执行强制类型转换，将vtable从 `usize` 类型转为 `*const usize` 类型
        let v: *const usize = vtable as *const () as *const usize;
        // 打印出指针 v 指向的内存区间的值
        println!("data in vtable [0x{:x}, 0x{:x}, 0x{:x}, 0x{:x}]",
                 *v, *v.offset(1), *v.offset(2), *v.offset(3));
    }
}

#[test]
fn test_dst() {
    let duck = Duck { v1: 111 };
    let p_duck = &duck;
    println!("size of Sized p_duck:{}", mem::size_of_val(&p_duck));
    // fat pointer
    let p_bird = p_duck as &dyn Bird;
    println!("size of ?Size dyn p_bird:{}", mem::size_of_val(&p_bird));
    // 获取函数地址
    let duck_fly: usize = Duck::fly as *const () as usize;
    let duck_beep: usize = Duck::beep as *const () as usize;
    // 获取函数地址
    let swan_fly: usize = Swan::fly as *const () as usize;
    //
    println!("Duck::fly 0x{:x}", duck_fly);
    println!("Swan::fly 0x{:x}", swan_fly);
    //
    print_trait_object(p_bird);
    let swan = Swan { v2: 100 };
    print_trait_object(&swan as &dyn Bird);
}