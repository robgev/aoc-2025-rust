use crate::utils::{num_of_digits, to_u_num};
use std::fs;

fn test_number(num: usize) -> bool {
    let digits = num_of_digits(num);
    let mut pow = 1;
    while pow < digits {
        let pattern = num % 10usize.pow(pow as u32);
        let mut rest = num / 10usize.pow(pow as u32);
        while rest > 0 {
            let next = rest % 10usize.pow(pow as u32);
            if next != pattern {
                break;
            }
            rest /= 10usize.pow(pow as u32);
        }

        if rest == 0 {
            return digits % pow == 0;
        }
        pow += 1;
    }

    return pow < digits;
}

fn solve_part_1() {
    let contents = fs::read_to_string("./src/input.txt").expect("Should read the file");
    let mut answer = 0;

    contents.split(',').for_each(|range| {
        let range_nums: Vec<usize> = range.split('-').map(to_u_num).collect();
        let start = range_nums[0];
        let end = range_nums[1];
        for i in start..=end {
            let num_digits = num_of_digits(i);
            if num_digits % 2 != 0 {
                continue;
            }
            let first = i / 10usize.pow(num_digits as u32 / 2);
            let second = i % 10usize.pow(num_digits as u32 / 2);
            if first == second {
                answer += i;
            }
        }
    });

    println!("Part 1 Answer: {answer} \n");
}

fn solve_part_2() {
    let contents = fs::read_to_string("./src/input.txt").expect("Should read the file");
    let mut answer = 0;

    contents.split(',').for_each(|range| {
        let range_nums: Vec<usize> = range.split('-').map(to_u_num).collect();
        let start = range_nums[0];
        let end = range_nums[1];
        for i in start..=end {
            if test_number(i) {
                answer += i;
            }
        }
    });

    println!("Part 2 Answer: {answer} \n");
}

pub fn solve() {
    self::solve_part_1();
    self::solve_part_2();
}
