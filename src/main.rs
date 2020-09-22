mod stackheap;

use std::mem;

static mut NAME: &str = "Rocky";

fn main() {
    println!("A rusty Hello to the world!");
    let a: u8 = 124;
    println!("a = {}", a);

    let mut b = 123456789;
    println!("b = {} and memory = {}", b, mem::size_of_val(&b));
    b = 1;
    println!("b = {} and memory = {}", b, mem::size_of_val(&b));

    let z: isize = 1;
    println!("z = {} and memory = {}", z, mem::size_of_val(&z));

    let hello_text = "Hello from variable!";
    fn say_hello() {
        let hello_text = "Hello from variable!";
        println!("Hello from a function INSIDE! {}", hello_text);
    }

    say_hello();

    show_unsafe();

    stackheap::stack_and_heap();
}

fn say_hello() {
    println!("Hello from a function OUTSIDE!");
}

fn show_unsafe() {
    unsafe {
        println!("{}", NAME);
    }
}


