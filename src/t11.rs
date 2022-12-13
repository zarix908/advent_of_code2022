use std::{
    fs::File,
    io::{BufRead, BufReader},
};

pub fn solve(reader: BufReader<File>) {
    let mut sum = 0;
    let mut max_sum = -1;

    for line in reader.lines() {
        let line = line.expect("read line failed");

        if !line.is_empty() {
            sum += line.parse::<i32>().unwrap();
            continue;
        }

        if sum > max_sum {
            max_sum = sum;
        }
        sum = 0;
    }

    println!("{}", max_sum);
}
