use std::{
    fs::File, 
    io::{BufReader, BufRead},
    cmp::Ordering
};

pub fn solve(reader: BufReader<File>) {
    let count = reader
        .lines()
        .map(|line| {
            let line = line.expect("read line failed");
            let pairs = line.split(",")
                .map(|p| 
                    p.split("-")
                        .map(|n| n.parse::<i32>().unwrap())
                        .collect::<Vec<_>>()
                )
                .collect::<Vec<Vec<_>>>();

            pairs
        })
        .filter(|p| !(p[0][0] > p[1][1] || p[0][1] < p[1][0]))
        .count();

    println!("{}", count);
}