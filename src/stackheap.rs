use std::mem;

pub fn stack_and_heap() {
    println!("Hello from stackheap!");
    let a = Box::new(1);
    println!("a = {} and memory occupies is {}", a, mem::size_of_val(&a));
}