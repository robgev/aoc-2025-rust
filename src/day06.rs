use crate::utils::to_u_num;
use std::{fs, usize};

fn solve_part_1() {
    let contents = fs::read_to_string("./src/input.txt").expect("Should read the file");
    let map: Vec<Vec<&str>> = contents
        .lines()
        .map(|line| line.split_whitespace().collect())
        .collect();
    let mut answer: usize = 0;

    let mut sign = 0;
    while map[sign][0].trim().parse::<i32>().is_ok() {
        sign += 1;
    }

    for col in 0..map[0].len() {
        let mut result = to_u_num(map[0][col]);
        for row in 1..sign {
            if map[sign][col].trim() == "*" {
                result *= to_u_num(map[row][col]);
            } else {
                result += to_u_num(map[row][col]);
            }
        }

        answer += result;
    }

    println!("Part 1 Answer: {answer} \n");
}

fn read_nums(grid: &Vec<Vec<char>>, offset: usize, width: usize, height: usize) -> Vec<usize> {
    let mut nums = Vec::new();
    for col in offset..(offset + width) {
        let mut result = 0;
        for row in 0..height {
            if grid[row][col] != ' ' {
                let digit = grid[row][col].to_digit(10).unwrap();
                result = 10 * result + digit;
            }
        }

        nums.push(result as usize);
    }

    return nums;
}

fn solve_part_2() {
    let contents = fs::read_to_string("./src/input.txt").expect("Should read the file");
    let map: Vec<Vec<char>> = contents
        .lines()
        .map(|line| line.chars().collect())
        .collect();
    let mut answer: usize = 0;

    let mut sign_row = 0;
    while map[sign_row][0] != '*' && map[sign_row][0] != '+' {
        sign_row += 1;
    }

    let mut sign_cols = Vec::new();

    // Signs are aligned at 0th position of the problem in the grid
    for i in 0..map[sign_row].len() {
        if map[sign_row][i] == '*' || map[sign_row][i] == '+' {
            sign_cols.push(i);
        }
    }

    // Now we know problem start cols, so let's go over each problem
    for i in 0..sign_cols.len() {
        // Careful for the last problem's case
        let problem_width = if i == sign_cols.len() - 1 {
            map[0].len() - sign_cols[i]
        } else {
            sign_cols[i + 1] - sign_cols[i]
        };

        // Delegate the grid parse work
        let parsed_nums = read_nums(&map, sign_cols[i], problem_width, sign_row);
        let mut result = parsed_nums[0];
        for j in 1..parsed_nums.len() {
            if parsed_nums[j] > 0 {
                if map[sign_row][sign_cols[i]] == '*' {
                    result *= parsed_nums[j];
                } else {
                    result += parsed_nums[j];
                }
            }
        }

        answer += result;
    }

    println!("Part 2 Answer: {answer} \n");
}

pub fn solve() {
    self::solve_part_1();
    self::solve_part_2();
}
