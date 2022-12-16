#[macro_use]
extern crate lazy_static;

mod t11;
mod t12;
mod t21;
mod t22;
mod t31;
mod t32;
mod t41;
mod t51;
mod t52;
mod t61;
mod t62;
mod t71;

use std::{fs::File, io::BufReader};

use t71::solve;

fn main() {
    let file = File::open("./data/7.txt").expect("open input file failed");
    let reader = BufReader::new(file);
    solve(reader);
}
