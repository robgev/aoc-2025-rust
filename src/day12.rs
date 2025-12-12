use std::fs;

use crate::utils::to_u_num;

struct Task {
    region_area: usize,
    presents_area: usize,
}

fn solve_part_1() {
    let contents = fs::read_to_string("./src/input.txt").expect("Should read the file");
    let sections: Vec<&str> = contents.split("\n\n").collect();

    let (shape_blocks, task_block) = sections.split_at(sections.len() - 1);

    let shape_areas: Vec<usize> = shape_blocks
        .iter()
        .map(|s| s.chars().filter(|&c| c == '#').count())
        .collect();

    let answer = task_block[0]
        .lines()
        .filter(|line| {
            let (dim_part, counts_part) = line.split_once(": ").expect("Invalid format");

            let (w, h) = dim_part.split_once('x').expect("Invalid dims");
            let region_area = to_u_num(w) * to_u_num(h);

            let presents_area: usize = counts_part
                .split_whitespace()
                .enumerate()
                .map(|(shape_idx, count_str)| to_u_num(count_str) * shape_areas[shape_idx])
                .sum();

            presents_area <= region_area
        })
        .count();

    println!("Part 1 Answer: {answer} \n");
}

fn solve_part_2() {
    println!("Merry Christmas!");
}

pub fn solve() {
    solve_part_1();
    solve_part_2();
}
