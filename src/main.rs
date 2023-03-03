#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use]
extern crate rocket;

fn main() {
    println!("Hello, world!");

    rocket::ignite().launch();
}
