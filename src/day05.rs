use crate::utils::to_u_num;
use std::collections::HashSet;
use std::{fs, usize};

fn solve_part_1() {
    let contents = fs::read_to_string("./src/input.txt").expect("Should read the file");
    let mut content_parts = contents.split_terminator("\n\n");
    let db: Vec<(usize, usize)> = content_parts
        .next()
        .unwrap()
        .lines()
        .map(|line| {
            let mut nums = line.split('-');
            let start = to_u_num(nums.next().unwrap());
            let end = to_u_num(nums.next().unwrap());
            (start, end)
        })
        .collect();

    let answer = content_parts.next().unwrap().lines().fold(0, |acc, line| {
        let id = to_u_num(&line);
        // return true if at least in one range
        let fresh = db.iter().any(|(start, end)| *start <= id && id <= *end);

        if fresh {
            acc + 1
        } else {
            acc
        }
    });

    println!("Part 1 Answer: {answer} \n");
}

fn solve_part_2() {
    let contents = fs::read_to_string("./src/input.txt").expect("Should read the file");
    let mut content_parts = contents.split_terminator("\n\n");
    let mut db: Vec<(usize, usize)> = content_parts
        .next()
        .unwrap()
        .lines()
        .map(|line| {
            let mut nums = line.split('-');
            let start = to_u_num(nums.next().unwrap());
            let end = to_u_num(nums.next().unwrap());
            (start, end)
        })
        .collect();

    for i in 0..db.len() {
        for j in (i + 1)..db.len() {
            let left = db[i];
            let right = db[j];
            if left.0 > 0 && right.0 > 0 {
                if left.0 <= right.1 && right.0 <= left.1 {
                    db[j] = (usize::min(left.0, right.0), usize::max(left.1, right.1));
                    db[i] = (0, 0)
                }
            }
        }
    }

    let answer = db.iter().fold(0, |acc, range| {
        if range.0 > 0 && range.1 > 0 {
            acc + (range.1 - range.0 + 1)
        } else {
            acc
        }
    });

    println!("Part 2 Answer: {answer} \n");
}

pub fn solve() {
    self::solve_part_1();
    self::solve_part_2();
}
