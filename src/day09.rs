use crate::utils::{calculate_area, to_u_num};
use std::{fs, isize, usize};

fn solve_part_1() {
    let contents = fs::read_to_string("./src/input.txt").expect("Should read the file");
    let vertices: Vec<(usize, usize)> = contents
        .lines()
        .map(|line| {
            let nums: Vec<usize> = line.split(",").map(to_u_num).collect();
            (nums[0], nums[1])
        })
        .collect();
    let mut answer = 0;

    for i in 0..vertices.len() {
        for j in (i + 1)..vertices.len() {
            let area = calculate_area(vertices[i], vertices[j]);
            if area > answer {
                answer = area;
            }
        }
    }

    println!("Part 1 Answer: {answer} \n");
}

fn is_rect_valid(
    min_x: isize,
    max_x: isize,
    min_y: isize,
    max_y: isize,
    edges: &Vec<((isize, isize), (isize, isize))>,
) -> bool {
    let center_x = (min_x as f64 + max_x as f64) / 2.0;
    let center_y = (min_y as f64 + max_y as f64) / 2.0;

    if !is_point_in_polygon(center_x, center_y, edges) {
        return false;
    }

    // Source: https://kishimotostudios.com/articles/aabb_collision/
    for &(start, end) in edges {
        if start.0 == end.0 {
            let edge_x = start.0;
            if edge_x > min_x && edge_x < max_x {
                let (e_min_y, e_max_y) = (start.1.min(end.1), start.1.max(end.1));

                if min_y.max(e_min_y) < max_y.min(e_max_y) {
                    return false;
                }
            }
        } else {
            let edge_y = start.1;
            if edge_y > min_y && edge_y < max_y {
                let (e_min_x, e_max_x) = (start.0.min(end.0), start.0.max(end.0));

                if min_x.max(e_min_x) < max_x.min(e_max_x) {
                    return false;
                }
            }
        }
    }

    true
}

fn is_point_in_polygon(px: f64, py: f64, edges: &Vec<((isize, isize), (isize, isize))>) -> bool {
    let mut inside = false;

    for &(p1, p2) in edges {
        let p1y = p1.1 as f64;
        let p2y = p2.1 as f64;
        let p1x = p1.0 as f64;
        let p2x = p2.0 as f64;

        let intersects_y = (p1y > py) != (p2y > py);

        if intersects_y {
            let intersect_x = p1x + (py - p1y) * (p2x - p1x) / (p2y - p1y);
            if px < intersect_x {
                inside = !inside;
            }
        }
    }

    inside
}

fn solve_part_2() {
    let contents = fs::read_to_string("./src/input.txt").expect("Should read the file");
    let vertices: Vec<(isize, isize)> = contents
        .lines()
        .map(|line| {
            let nums: Vec<isize> = line.split(",").map(|n| n.parse().unwrap()).collect();
            (nums[0], nums[1])
        })
        .collect();
    let mut edges = Vec::new();
    for i in 0..vertices.len() {
        let start = vertices[i];
        let end = vertices[(i + 1) % vertices.len()];
        edges.push((start, end));
    }

    let mut max_area = 0;

    for i in 0..vertices.len() {
        for j in (i + 1)..vertices.len() {
            let p1 = vertices[i];
            let p2 = vertices[j];

            let width = (p1.0 - p2.0).abs() + 1;
            let height = (p1.1 - p2.1).abs() + 1;
            let area = width * height;

            if area <= max_area {
                continue;
            }

            let (min_x, max_x) = (p1.0.min(p2.0), p1.0.max(p2.0));
            let (min_y, max_y) = (p1.1.min(p2.1), p1.1.max(p2.1));

            if is_rect_valid(min_x, max_x, min_y, max_y, &edges) {
                max_area = area;
            }
        }
    }

    println!("Part 2 Answer: {max_area} \n");
}

pub fn solve() {
    self::solve_part_1();
    self::solve_part_2();
}
