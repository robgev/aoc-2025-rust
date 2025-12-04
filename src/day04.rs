use crate::utils::is_in_bounds;
use std::fs;

fn count_neighbors(map: &Vec<Vec<char>>, (sr, sc): (i32, i32)) -> usize {
    let mut count = 0;
    for (d_row, d_col) in [
        (-1, -1),
        (-1, 0),
        (-1, 1),
        (0, -1),
        (0, 1),
        (1, -1),
        (1, 0),
        (1, 1),
    ] {
        let next_row = sr + d_row;
        let next_col = sc + d_col;
        if is_in_bounds((map.len(), map[0].len()), (next_row, next_col)) {
            if map[next_row as usize][next_col as usize] == '@' {
                count += 1;
            }
        }
    }

    count
}

fn clean_once(map: &mut Vec<Vec<char>>, replace: bool) -> i32 {
    let mut cleaned = 0;
    for i in 0..map.len() {
        for j in 0..map[0].len() {
            if map[i as usize][j as usize] == '@' {
                if count_neighbors(&map, (i as i32, j as i32)) < 4 {
                    if replace {
                        map[i as usize][j as usize] = 'x';
                    }
                    cleaned += 1;
                }
            }
        }
    }

    cleaned
}

fn solve_part_1() {
    let contents = fs::read_to_string("./src/input.txt").expect("Should read the file");
    let mut map: Vec<Vec<char>> = contents
        .lines()
        .map(|line| line.chars().collect())
        .collect();

    let answer = clean_once(&mut map, false);

    println!("Part 1 Answer: {answer} \n");
}

fn solve_part_2() {
    let contents = fs::read_to_string("./src/input.txt").expect("Should read the file");
    let mut map: Vec<Vec<char>> = contents
        .lines()
        .map(|line| line.chars().collect())
        .collect();
    let mut answer = 0;

    loop {
        let cleaned = clean_once(&mut map, true);
        if cleaned > 0 {
            answer += cleaned;
        } else {
            break;
        }
    }

    println!("Part 2 Answer: {answer} \n");
}

pub fn solve() {
    self::solve_part_1();
    self::solve_part_2();
}
