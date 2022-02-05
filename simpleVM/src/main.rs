#![allow(unused_imports)]
#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
#![allow(dead_code)]
#![allow(unused_variables)]

use std::{collections::HashMap, fmt::Result};

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
fn read(a: &Address) -> Result<String, i32> {
    if a.loc < a.memmap.capacity() as usize { Ok(a.memmap.get(&a.loc)) }
    else { println!("SegFault"); Err(-1);  }
}


fn main() {
    println!("Hello, world!");
    let addr = Register::default;
}
