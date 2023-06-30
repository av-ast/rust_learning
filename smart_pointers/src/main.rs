#[derive(Debug)]
enum List {
    Cons(i32, Box<List>),
    Nil,
}

use List::{Cons, Nil};
use core::ops::Deref;

struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &T {
        &self.0
    }
}

fn main() {
    let b = Box::new(5);
    println!("b = {}", b);

    let list = Cons(1, Box::new(Cons(2, Box::new(Nil))));
    println!("list {:?}", list);

    let x = 5;
    let y = MyBox::new(x);
    println!("y = {}", *y);

    let m = MyBox::new(String::from("Rust"));
    hello(&m);
    // deref coersion is working here
    // &MyBox<String> -> &String -> &str
}

fn hello(name: &str) {
    println!("Hello, {}!", name);
}
