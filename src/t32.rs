use std::{
    collections::HashSet,
    fs::File,
    io::{BufRead, BufReader},
};

pub fn solve(reader: BufReader<File>) {
    let sum: u32 = reader
        .lines()
        .collect::<Result<Vec<_>, _>>()
        .expect("read line failed")
        .chunks(3)
        .map(|lines| {
            lines
                .into_iter()
                .map(|line| line.chars().collect::<HashSet<_>>())
                .reduce(|acc, s| acc.intersection(&s).map(|i| *i).collect())
                .unwrap()
                .into_iter()
                .next()
                .map(|c| c as u32 - if c.is_uppercase() { 38 } else { 96 })
                .unwrap()
        })
        .sum();
    println!("{}", sum);
}
