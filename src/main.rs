#[macro_use]
extern crate lazy_static;

#[global_allocator]
static ALLOC: jemallocator::Jemalloc = jemallocator::Jemalloc;

mod t11;
mod t12;
mod t21;
mod t22;
mod t31;
mod t32;
mod t41;
mod t42;
mod t51;
mod t52;
mod t61;
mod t62;
mod t71;
mod t72;
mod t81;
mod mem;

use std::{fs::File, io::BufReader};

use t81::solve;

fn main() {
    let file = File::open("./data/8.txt").expect("open input file failed");
    let reader = BufReader::new(file);
    solve(reader);
}