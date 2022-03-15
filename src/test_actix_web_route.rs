#![allow(unused)]

//! 本模块模仿actix-web的Route::to函数
//! Route::to 能够接受任意一个函数作为路由处理器
//! 并且函数的参数可以是`任意顺序`的actix-web/types模块下的类型
//! 实现时,巧妙结合了宏跟泛型

use std::marker::PhantomData;

trait FromSomething {
    fn from_sth(name: &str) -> Self;
}

impl FromSomething for () {
    fn from_sth(_name: &str) {}
}

impl FromSomething for String {
    fn from_sth(name: &str) -> Self {
        name.into()
    }
}

macro_rules! def_data {
    ($($t:ident),*) => {
        $(
        #[derive(Debug, Ord, PartialOrd, Eq, PartialEq)]
        struct $t(String);
        impl FromSomething for $t{
            fn from_sth(name: &str) -> Self {
                $t(name.into())
            }
        }
        )*
    };
}

def_data!(Data0,Data1,Data2,Data3);

trait FnWrapper<T, R> {
    fn call(&self, _: T) -> R;
}

impl<T, R> FnWrapper<(), R> for T where T: 'static + Fn() -> R {
    fn call(&self, _: ()) -> R {
        println!("fn wrapper: no param");
        self()
    }
}

macro_rules! extend_fn_wrapper {
    ($(($t:tt,$ty:ident)),*) => {
        impl <$($ty:FromSomething,)*> FromSomething for ($($ty,)*) {

            fn from_sth(name: &str) -> Self {
                const N :usize = <[i32]>::len(&[$($t as i32),*]);
                let v: Vec<&str> = name.split(",").collect();
                if v.len() < N {
                    panic!("参数不足:期望:{},实际:{}", N, v.len());
                }
                unsafe{
                    ($($ty::from_sth(v.get_unchecked($t)),)*)
                }
            }
        }

        impl <T,R,$($ty,)*> FnWrapper<($($ty,)*),R> for T
            where T:'static + Fn($($ty,)*)->R {
            fn call(&self, p: ($($ty,)*)) -> R {
                self($(p.$t,)*)
            }
        }
    };
}

extend_fn_wrapper!((0,A));
extend_fn_wrapper!((0,A),(1,B));
extend_fn_wrapper!((0,A),(1,B),(2,C));
extend_fn_wrapper!((0,A),(1,B),(2,C),(3,D));

struct Invoker<FP, FR, Inner> {
    handle: Inner,
    _p: PhantomData<(FP, FR)>,
}

impl<FP, FR, Inner: FnWrapper<FP, FR>> Invoker<FP, FR, Inner> {
    fn new(inner: Inner) -> Self {
        Self {
            handle: inner,
            _p: PhantomData,
        }
    }
}

impl<FP: FromSomething, FR, Inner: FnWrapper<FP, FR>> Service for Invoker<FP, FR, Inner> {
    type Output = FR;

    fn call(&self, s: &'static str) -> Self::Output {
        self.handle.call(FP::from_sth(s))
    }
}

trait Service {
    type Output;

    fn call(&self, s: &'static str) -> Self::Output;
}

struct Route {
    inner: Box<dyn Service<Output=std::io::Result<i32>>>,
}

fn do_work1(d0: Data0) -> std::io::Result<i32> {
    println!("do_work1: d0:{:?}", d0);
    Ok(0)
}


fn do_work2(d0: Data0, d1: Data3, d2: Data2, s: String) -> std::io::Result<i32> {
    println!("do_work2: d0:{:?},d1:{:?},d2:{:?},s:{}", d0, d1, d2, s);
    Ok(0)
}

impl Route {
    fn new<F, FP>(f: F) -> Self
        where F: 'static + FnWrapper<FP, std::io::Result<i32>>,
              FP: 'static + FromSomething {
        let invoker = Invoker::new(f);
        Route {
            inner: Box::new(invoker)
        }
    }
}

#[test]
fn test() {
    let v = Route::new(do_work1);
    v.inner.call("1,2,3,4").unwrap();
    let v = Route::new(do_work2);
    v.inner.call("aaa,bbb,ccc,ddd").unwrap();
    // 模拟参数不足的情况
    //let v = Route::new(do_work2);
    //v.inner.call("aaa").unwrap();
}

