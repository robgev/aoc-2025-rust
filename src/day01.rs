use crate::utils::to_num;
use std::fs;

fn solve_part_1() {
    let contents = fs::read_to_string("./src/input.txt").expect("Should read the file");
    let mut answer = 0;
    let mut number = 50;

    contents.lines().for_each(|line| {
        let dir = line.chars().nth(0).unwrap();
        let current_sign = if dir == 'L' { -1 } else { 1 };
        let amount = to_num(&line[1..]);

        number += current_sign * amount + 100;
        number %= 100;

        if number == 0 {
            answer += 1;
        }
    });
    println!("Part 1 Answer: {answer} \n");
}

fn solve_part_2() {
    let contents = fs::read_to_string("./src/input.txt").expect("Should read the file");
    let mut answer = 0;
    let mut number = 50;

    contents.lines().for_each(|line| {
        let dir = line.chars().nth(0).unwrap();
        let amount = to_num(&line[1..]);

        for _ in 0..amount {
            if dir == 'L' {
                number = (number - 1 + 100) % 100;
            } else {
                number = (number + 1) % 100;
            }
            if number == 0 {
                answer += 1;
            }
        }
    });

    println!("Part 2 Answer: {answer}");
}

pub fn solve() {
    self::solve_part_1();
    self::solve_part_2();
}
