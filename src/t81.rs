use std::{
    fs::File,
    io::{BufRead, BufReader},
    cmp::max, borrow::Borrow
};

pub fn solve(reader: BufReader<File>) {
    let mut rows: Vec<Vec<(i32, i32, i32, i32)>> = Vec::new();

    for line in reader.lines() {
        let line = line.expect("read line failed");
        
        if rows.len() == 0 {
            rows.push(vec![(-1, -1, -1, -1); line.len() + 2]);
        }

        let mut tree_heights: Vec<(i32, i32, i32, i32)> = vec![(-1, -1, -1, -1)];
        for (col, height) in line.chars().map(|c| c.to_digit(10).unwrap()).enumerate() {
            let left = tree_heights[tree_heights.len() - 1].0;
            let top = rows[rows.len() - 1][col + 1].1;

            tree_heights.push((
                max(left, height as i32),
                max(top, height as i32), 
                height as i32, -1
            ));
        }

        tree_heights.push((-1, -1, -1, -1));
        rows.push(tree_heights);
    }
    rows.push(vec![(-1, -1, -1, -1); rows[0].len() + 2]);

    for row in (1..rows.len() - 1).rev() {
        for column in (1..rows[0].len() - 1).rev() {
            let right = rows[row][column + 1].2;
            let bottom = rows[row + 1][column].3;
            let height = rows[row][column].2;

            rows[row][column].2 = max(right, height);
            rows[row][column].3 = max(bottom, height);
        } 
    }

    println!("{:?}", rows);
}
