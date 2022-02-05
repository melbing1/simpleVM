#![allow(unused_imports)]
#![allow(dead_code)]

use std::collections::HashMap;

#[derive(Default)]
struct Register {
    ax: u32,
    bx: u32,
    cx: u32,
    dx: u32,
}

#[derive(Default)]
struct Address {
    loc: u32,
    memmap: HashMap<u32, String>,
}

// Instructions
fn read(a: &Address) -> String {
    if a.loc < a.memmap.capacity() as usize { a.memmap.get(&a.loc) }
    else { println!("SegFault") }
}


fn main() {
    println!("Hello, world!");
    let addr = Register::default;
}
