use crate::utils::{find_start_loc, is_in_bounds, print_grid};
use std::{
    collections::{HashMap, HashSet, VecDeque},
    fs, usize,
};

fn solve_part_1() {
    let contents = fs::read_to_string("./src/input.txt").expect("Should read the file");
    let map: Vec<Vec<char>> = contents.lines().map(|l| l.chars().collect()).collect();
    let start_loc = find_start_loc(&map, 'S');
    let bounds = (map.len(), map[0].len());
    let mut pending_traces: VecDeque<(i32, i32)> = VecDeque::from([start_loc]);
    let mut seen: HashSet<(i32, i32)> = HashSet::new();
    let mut answer = 0;

    while let Some((row, col)) = pending_traces.pop_front() {
        if !is_in_bounds(bounds, (row, col)) || seen.contains(&(row, col)) {
            continue;
        }

        seen.insert((row, col));

        match map[row as usize][col as usize] {
            '^' => {
                answer += 1;

                pending_traces.push_back((row, col - 1));
                pending_traces.push_back((row, col + 1));
            }
            _ => {
                pending_traces.push_back((row + 1, col));
            }
        }
    }

    println!("Part 1 Answer: {answer} \n");
}

fn count_timelines(
    pos: (i32, i32),
    map: &Vec<Vec<char>>,
    bounds: (usize, usize),
    memo: &mut HashMap<(i32, i32), u64>,
) -> u64 {
    let (row, col) = pos;

    if !is_in_bounds(bounds, pos) {
        return 1;
    }

    if let Some(&count) = memo.get(&pos) {
        return count;
    }

    let result = match map[row as usize][col as usize] {
        '^' => {
            let left_path = count_timelines((row, col - 1), map, bounds, memo);
            let right_path = count_timelines((row, col + 1), map, bounds, memo);
            left_path + right_path
        }
        _ => count_timelines((row + 1, col), map, bounds, memo),
    };

    memo.insert(pos, result);
    result
}

fn solve_part_2() {
    let contents = fs::read_to_string("./src/input.txt").expect("Should read the file");
    let map: Vec<Vec<char>> = contents.lines().map(|l| l.chars().collect()).collect();
    let start_loc = find_start_loc(&map, 'S');

    let mut memo: HashMap<(i32, i32), u64> = HashMap::new();
    let bounds = (map.len(), map[0].len());

    let answer = count_timelines(start_loc, &map, bounds, &mut memo);

    println!("Part 2 Answer: {answer} \n");
}

pub fn solve() {
    self::solve_part_1();
    self::solve_part_2();
}
