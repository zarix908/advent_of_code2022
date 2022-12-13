use std::{
    fs::File,
    io::{BufRead, BufReader},
};

pub fn solve(reader: BufReader<File>) {
    let mut sum = 0;
    let mut top = [-1, -1, -1, -1];

    for line in reader.lines() {
        let line = line.expect("read line failed");

        if !line.is_empty() {
            sum += line.parse::<i32>().unwrap();
            continue;
        }

        top[0] = sum;
        for i in 0..top.len() - 1 {
            if top[i] < top[i + 1] {
                break;
            }

            (top[i], top[i + 1]) = (top[i + 1], top[i]);
        }
        sum = 0;
    }

    println!("{}", top[1..].into_iter().sum::<i32>());
}
