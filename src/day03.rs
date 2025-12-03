use crate::utils::to_num;
use std::fs;

fn solve_part_1() {
    let contents = fs::read_to_string("./src/input.txt").expect("Should read the file");
    let answer = contents.lines().fold(0, |acc, line| {
        let mut max = 0;

        for left in 0..line.len() {
            for right in left + 1..line.len() {
                let current = 10 * to_num(&line[left..left + 1]) + to_num(&line[right..right + 1]);
                if current > max {
                    max = current;
                }
            }
        }

        acc + max
    });

    println!("Part 1 Answer: {answer} \n");
}

fn max_for(input: &str) -> u64 {
    let digits: Vec<u32> = input.chars().map(|c| c.to_digit(10).unwrap()).collect();
    let n = digits.len();
    let mut stack: Vec<u32> = Vec::with_capacity(12);

    digits.iter().enumerate().for_each(|(i, &digit)| {
        while let Some(&top) = stack.last() {
            let remaining_input = n - i;
            let current_stack_len = stack.len();

            if digit > top && (current_stack_len - 1 + remaining_input >= 12) {
                stack.pop();
            } else {
                break;
            }
        }

        if stack.len() < 12 {
            stack.push(digit);
        }
    });

    stack.iter().fold(0, |acc, &d| acc * 10 + d as u64)
}

fn solve_part_2() {
    let contents = fs::read_to_string("./src/input.txt").expect("Should read the file");
    let answer = contents.lines().fold(0, |acc, line| acc + max_for(line));

    println!("Part 2 Answer: {answer} \n");
}

pub fn solve() {
    self::solve_part_1();
    self::solve_part_2();
}
