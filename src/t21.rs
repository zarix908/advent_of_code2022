use std::{
    collections::HashMap,
    fs::File,
    io::{BufRead, BufReader},
};

pub fn solve(reader: BufReader<File>) {
    let mut score = 0;

    let orders = HashMap::from([("A", "Z"), ("B", "X"), ("C", "Y")]);

    let shape_scores = HashMap::from([("A", 1), ("B", 2), ("C", 3), ("X", 1), ("Y", 2), ("Z", 3)]);

    for line in reader.lines() {
        let line = line.expect("read line failed");
        let line: Vec<&str> = line.split(" ").collect();

        let shape_score = shape_scores.get(line[1]).unwrap();
        score += shape_score;

        if shape_scores.get(line[0]).unwrap() - shape_score == 0 {
            score += 3;
            continue;
        }

        if orders.get(line[0]).unwrap() != &line[1] {
            score += 6;
        }
    }

    println!("{}", score);
}
