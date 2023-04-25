use cacher::*;

fn main() {
    let mut cacher = Cacher::new(|arg| {
        println!("Calculation body");
        arg + 5
    });

    println!("First call. {}", cacher.value(1));
    println!("Second call. {}", cacher.value(1));
}
