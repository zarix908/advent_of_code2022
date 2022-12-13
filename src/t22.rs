use std::{
    collections::HashMap,
    fs::File,
    io::{BufRead, BufReader},
};

pub fn solve(reader: BufReader<File>) {
    let mut score = 0;

    let loose = HashMap::from([("A", "Z"), ("B", "X"), ("C", "Y")]);
    let win = HashMap::from([("A", "Y"), ("B", "Z"), ("C", "X")]);

    let shape_scores = HashMap::from([("A", 1), ("B", 2), ("C", 3), ("X", 1), ("Y", 2), ("Z", 3)]);

    for line in reader.lines() {
        let line = line.expect("read line failed");
        let line: Vec<&str> = line.split(" ").collect();

        let action = line[1];
        let (shape, tour_score) = match action {
            "X" => (loose.get(line[0]), 0),
            "Y" => (Some(&line[0]), 3),
            "Z" => (win.get(line[0]), 6),
            _ => unreachable!(),
        };
        score += tour_score;
        score += shape_scores.get(shape.unwrap()).unwrap();
    }

    println!("{}", score);
}
