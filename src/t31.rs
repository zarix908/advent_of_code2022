use std::{
    collections::HashSet,
    fs::File,
    io::{BufRead, BufReader},
};

pub fn solve(reader: BufReader<File>) {
    let sum: u32 = reader
        .lines()
        .map(|line| {
            let line = line.expect("read line failed");
            let items: HashSet<_> = line[0..line.len() / 2].chars().collect();
            let common = line[line.len() / 2..]
                .chars()
                .find(|i| items.contains(i))
                .unwrap();
            common as u32 - if common.is_uppercase() { 38 } else { 96 }
        })
        .sum();
    println!("{}", sum);
}
