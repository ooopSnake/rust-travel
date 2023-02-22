// 到处到当前空间
use school::class_show;
use school::teacher_show;

mod school {
    pub use class::private_class::private_show;
    pub(super) use class::show as class_show;
    pub(super) use teacher::show as teacher_show;

    mod teacher {
        pub fn show() {
            println!("teacher::show");
        }
    }

    mod class {
        pub fn show() {
            println!("class::show");
        }

        // super::super 为 school. private_class 只对 school 可见
        pub(in super::super) mod private_class {
            // 这里是pub可见,所以可以从school中导出
            pub fn private_show() {
                println!("this is fucking private show!!!")
            }
        }
    }
}

#[test]
fn test() {
    self::teacher_show();
    self::class_show();
    school::private_show();
}