use std::{
    collections::LinkedList,
    fs::File,
    io::{BufRead, BufReader},
};

pub fn solve(reader: BufReader<File>) {
    let mut stacks = vec![LinkedList::<u8>::new(); 9];
    let mut lines = reader.lines();

    for line in lines.by_ref().take(8) {
        let line = line.expect("read line failed");
        for i in 0..9 {
            let c = line.as_bytes()[1 + i * 4];
            if !c.is_ascii_whitespace() {
                stacks[i].push_front(c);
            }
        }
    }

    for line in lines.skip(2) {
        let line = line
            .expect("read line failed")
            .split(" ")
            .map(|s| s.to_owned())
            .collect::<Vec<String>>();
        let count: usize = line[1].parse().unwrap();
        let from = line[3].parse::<usize>().unwrap() - 1;
        let to = line[5].parse::<usize>().unwrap() - 1;

        let split_index = stacks[from].len() - count;
        let mut splitted = stacks[from].split_off(split_index);
        stacks[to].append(&mut splitted);
    }

    let res = String::from_utf8(stacks.into_iter()
        .map(|mut s| s.pop_back().unwrap()).collect::<Vec<u8>>());
    println!("{}", res.expect("convert to utf8 failed"));
}
