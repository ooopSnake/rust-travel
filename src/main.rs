#![allow(unused)]

pub mod assign;
pub mod mod_example;
pub mod closure;
pub mod dst;
pub mod enum_example;
pub mod fn_shadow;
pub mod lifetime;
pub mod async_executor;
pub mod custom_vec;
pub mod actix_web_route;
pub mod fill_default;
pub mod linked_list;
pub mod futex;
pub mod gat;

#[macro_use]
pub mod macro_example;

pub mod memory;
pub mod net;


pub fn main() {
    println!("nothing is true,everything is permitted")
}