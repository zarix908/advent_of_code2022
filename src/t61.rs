use std::{
    fs::File,
    io::{BufReader},
    collections::HashMap,
    io::Read
};

pub fn solve(mut reader: BufReader<File>) {
    let mut l = 0;
    let mut r = 0;
    let mut last_pos = HashMap::new();
    
    let mut byte = [0u8; 1];
    while let Ok(()) = reader.read_exact(&mut byte) {
        r += 1;
        if let Some(pos) = last_pos.get(&byte[0]) {
            if *pos > l {
                l = *pos;
            }
        }
        last_pos.insert(byte[0], r - 1);

        if r - l == 4 {
            println!("{}", r);
            break;
        }
    }
}
