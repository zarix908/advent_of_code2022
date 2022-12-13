use std::{
    cmp::Ordering,
    fs::File,
    io::{BufRead, BufReader},
};

pub fn solve(reader: BufReader<File>) {
    let count = reader
        .lines()
        .map(|line| {
            let line = line.expect("read line failed");
            let pairs = line
                .split(",")
                .map(|p| {
                    p.split("-")
                        .map(|n| n.parse::<i32>().unwrap())
                        .collect::<Vec<_>>()
                })
                .collect::<Vec<Vec<_>>>();

            (pairs[0][0].cmp(&pairs[1][0]), pairs[0][1].cmp(&pairs[1][1]))
        })
        .filter(|ords| ords.0 != ords.1 || (ords.0 == Ordering::Equal && ords.1 == Ordering::Equal))
        .count();

    println!("{}", count);
}
