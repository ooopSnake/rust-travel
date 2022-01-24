#![allow(unused)]

//! 自己动手,实现一个异步executor

use std::future::Future;
use std::marker::PhantomData;
use std::ops::{Deref, DerefMut};
use std::pin::Pin;
use std::ptr::NonNull;
use std::sync::{Arc, mpsc};
use std::sync::atomic::{AtomicBool, AtomicU32, Ordering};
use std::task::{Context, Poll, RawWaker, RawWakerVTable, Waker};
use std::time::Duration;

struct RefMem<T: ?Sized> {
    ptr: NonNull<T>,
    counter: NonNull<AtomicU32>,
    _p: PhantomData<T>,
}

impl<T: ?Sized> RefMem<T> {
    fn new(t: Box<T>) -> Self {
        unsafe {
            RefMem {
                ptr: NonNull::new_unchecked(Box::leak(t)),
                counter: NonNull::new(
                    Box::leak(Box::new(
                        AtomicU32::new(1)))).unwrap(),
                _p: PhantomData,
            }
        }
    }

    fn incr(&self) {
        unsafe {
            let v = self.counter.as_ref()
                .fetch_add(1, Ordering::Relaxed);
            println!("incr:{}", v + 1)
        }
    }

    fn decr(&self) -> bool {
        1u32 == unsafe {
            let v = self.counter.as_ref()
                .fetch_sub(1, Ordering::Relaxed);
            println!("decr:{}", v - 1);
            v
        }
    }
}

impl<T: ?Sized> Clone for RefMem<T> {
    fn clone(&self) -> Self {
        println!("call clone");
        self.incr();
        unsafe {
            RefMem {
                ptr: NonNull::new_unchecked(self.ptr.as_ptr()),
                counter: NonNull::new_unchecked(self.counter.as_ptr()),
                _p: PhantomData,
            }
        }
    }
}

impl<T: ?Sized> Deref for RefMem<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        unsafe {
            &*(self.ptr.as_ptr())
        }
    }
}

impl<T: ?Sized> DerefMut for RefMem<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe {
            &mut *(self.ptr.as_ptr())
        }
    }
}

impl<T: ?Sized> Drop for RefMem<T> {
    fn drop(&mut self) {
        let shall_drop = self.decr();
        if shall_drop {
            unsafe {
                std::ptr::drop_in_place(self.ptr.as_ptr());
            }
        }
    }
}

struct LocalWaker {
    name: String,
    send: mpsc::Sender<RefMem<dyn Future<Output=()>>>,
    f: RefMem<dyn Future<Output=()>>,
}

impl LocalWaker {
    unsafe fn v_clone(data: *const ()) -> RawWaker {
        let walker = &mut *(data as *mut LocalWaker);
        println!("clone:{}", &walker.name);
        LocalWaker {
            name: format!("  **cloned** from: [{}]", &walker.name).to_string(),
            send: walker.send.clone(),
            f: walker.f.clone(),
        }.into()
    }

    unsafe fn v_wake(data: *const ()) {
        let walker = &mut *(data as *mut LocalWaker);
        walker.send.send(walker.f.clone()).unwrap();
        println!("wake:{}", &walker.name);
        // manual drop
        Self::v_drop(data)
    }

    unsafe fn v_wake_by_ref(data: *const ()) {
        let walker = &mut *(data as *mut LocalWaker);
        println!("wake ref:{}", &walker.name);
        walker.send.send(walker.f.clone()).unwrap();
    }

    unsafe fn v_drop(data: *const ()) {
        let walker = &mut *(data as *mut LocalWaker);
        println!("drop invoked:{}", &walker.name);
        std::ptr::drop_in_place(walker as *mut LocalWaker)
    }

    fn make_vt() -> &'static RawWakerVTable {
        static VT: RawWakerVTable = RawWakerVTable::new(
            LocalWaker::v_clone,
            LocalWaker::v_wake,
            LocalWaker::v_wake_by_ref,
            LocalWaker::v_drop);
        &VT
    }
}

impl Into<RawWaker> for LocalWaker {
    fn into(self) -> RawWaker {
        let p = Box::leak(Box::new(self)) as *mut Self;
        RawWaker::new(p as *const (), LocalWaker::make_vt())
    }
}

impl Into<Waker> for LocalWaker {
    fn into(self) -> Waker {
        unsafe {
            Waker::from_raw(Into::<RawWaker>::into(self))
        }
    }
}

struct Delay {
    n: Duration,
    finished: Arc<AtomicBool>,
}

impl Drop for Delay {
    fn drop(&mut self) {
        println!("delay drop!")
    }
}

impl Future for Delay {
    type Output = ();

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let state = self.finished.load(Ordering::Relaxed);
        if !state {
            let waker = cx.waker().clone();
            let (flag, dur) = (
                self.finished.clone(),
                self.n.clone());
            std::thread::spawn(move || {
                std::thread::sleep(dur);
                flag.as_ref().store(true, Ordering::Relaxed);
                waker.wake();
            });
            println!("-------> pending");
            Poll::Pending
        } else {
            println!("-------> ready");
            Poll::Ready(())
        }
    }
}

fn mdelay(nsec: u64) -> Delay {
    Delay {
        n: Duration::from_secs(nsec),
        finished: Default::default(),
    }
}

async fn heavy_calc_task() -> i32 {
    mdelay(2).await;
    return 100;
}

async fn worker() {
    mdelay(2).await;
    println!("jobs 1 done");
    let v = heavy_calc_task().await;
    println!("jobs 2 done:{}", v)
}

fn logic() {
    let stop_flag = Arc::new(AtomicBool::new(false));
    let fut: RefMem<dyn Future<Output=()>> = RefMem::new(
        Box::new(worker()));
    let (sender, receiver) = mpsc::channel();
    sender.send(fut).unwrap();
    let cloned_stop_flag = stop_flag.clone();
    sender.send(RefMem::new(
        Box::new(async move {
            mdelay(5).await;
            cloned_stop_flag.store(true, Ordering::Relaxed);
        }))).unwrap();
    let mut loop_idx = 0;
    loop {
        loop_idx += 1;
        let x = receiver.recv().unwrap();
        let walk = LocalWaker {
            name: format!(" [loop::{}]", loop_idx).into(),
            send: sender.clone(),
            f: x,
        };
        let r = unsafe {
            let r = Pin::new_unchecked(&mut *(walk.f.ptr.as_ptr()))
                .poll(&mut Context::from_waker(&walk.into()));
            r
        };
        if r.is_pending() {
            println!("----------------> pending");
        } else {
            println!("----------------> ready");
        }
        if stop_flag.load(Ordering::Relaxed) {
            break;
        }
    }
}

#[test]
fn test_async_executor() {
    logic();
    println!("exit")
}


//
// fn call(d: Rc<dyn Future<Output=()>>) {
//     println!("call")
// }
//
// fn call2(d: RefMem<dyn Future<Output=()>>) {
//     println!("call")
// }
//
// fn main() {
//     let v = Rc::new(Delay {
//         n: Duration::from_secs(10),
//         finished: Arc::new(AtomicBool::default()),
//     });
//     call(v);
//     let v = RefMem::new(
//         Box::new(Delay {
//             n: Duration::from_secs(10),
//             finished: Arc::new(AtomicBool::default()),
//         })
//     );
//     call2(v);
// }