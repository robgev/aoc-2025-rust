use crate::utils::to_u_num;
use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashMap},
    fs,
};

fn distance(a: (usize, usize, usize), b: (usize, usize, usize)) -> usize {
    (a.0.abs_diff(b.0)).pow(2) + (a.1.abs_diff(b.1)).pow(2) + (a.2.abs_diff(b.2)).pow(2)
}

struct UnionFind {
    parents: Vec<usize>,
    size: Vec<usize>,
}

impl UnionFind {
    fn new(n: usize) -> Self {
        UnionFind {
            // Each box starts in their circuit
            parents: (0..n).collect(),
            // Of length 1
            size: vec![1; n],
        }
    }

    fn find_root(&mut self, i: usize) -> usize {
        if self.parents[i] == i {
            i
        } else {
            let root = self.find_root(self.parents[i]);
            // Unify the root for all boxes in this circuit
            self.parents[i] = root;
            root
        }
    }

    fn connect(&mut self, i: usize, j: usize) {
        let root_i = self.find_root(i);
        let root_j = self.find_root(j);

        if root_i == root_j {
            // println!("Already connected");
            return;
        }

        self.parents[root_i] = root_j;
        self.size[root_j] += self.size[root_i];
    }

    fn get_top_k(&mut self, k: usize) -> Vec<usize> {
        let mut sizes = HashMap::new();
        for i in 0..self.parents.len() {
            let root = self.find_root(i);
            sizes.insert(root, self.size[root]);
        }

        let mut size_list: Vec<usize> = sizes.into_values().collect();
        size_list.sort_unstable_by(|a, b| b.cmp(a));
        size_list.iter().take(k as usize).cloned().collect()
    }
}

fn solve_part_1() {
    let contents = fs::read_to_string("./src/input.txt").expect("Should read the file");
    let coordinates: Vec<(usize, usize, usize)> = contents
        .lines()
        .map(|line| {
            let nums: Vec<usize> = line.split(",").map(to_u_num).collect();
            (nums[0], nums[1], nums[2])
        })
        .collect();
    let mut pq: BinaryHeap<(Reverse<usize>, (usize, usize))> = BinaryHeap::new();
    let mut circuits = UnionFind::new(coordinates.len());

    for i in 0..coordinates.len() {
        for j in (i + 1)..coordinates.len() {
            pq.push((
                Reverse(distance(coordinates[i], coordinates[j])),
                (i as usize, j as usize),
            ));
        }
    }

    let limit = 1000;
    for _ in 0..limit {
        if let Some(next) = pq.pop() {
            let (_, (a, b)) = next;
            circuits.connect(a as usize, b as usize);
        }
    }

    let answer: usize = circuits.get_top_k(3).iter().product();

    println!("Part 1 Answer: {answer} \n");
}

fn solve_part_2() {
    let contents = fs::read_to_string("./src/input.txt").expect("Should read the file");
    let coordinates: Vec<(usize, usize, usize)> = contents
        .lines()
        .map(|line| {
            let nums: Vec<usize> = line.split(",").map(to_u_num).collect();
            (nums[0], nums[1], nums[2])
        })
        .collect();
    let mut pq: BinaryHeap<(Reverse<usize>, (usize, usize))> = BinaryHeap::new();
    let mut circuits = UnionFind::new(coordinates.len());

    for i in 0..coordinates.len() {
        for j in (i + 1)..coordinates.len() {
            pq.push((
                Reverse(distance(coordinates[i], coordinates[j])),
                (i as usize, j as usize),
            ));
        }
    }

    let mut last_connection = (0, 0);
    while *(circuits.get_top_k(3).first().unwrap()) < coordinates.len() {
        if let Some(next) = pq.pop() {
            let (_, (a, b)) = next;
            circuits.connect(a as usize, b as usize);
            last_connection = (a, b);
        }
    }

    let answer = coordinates[last_connection.0].0 * coordinates[last_connection.1].0;

    println!("Part 2 Answer: {answer} \n");
}

pub fn solve() {
    self::solve_part_1();
    self::solve_part_2();
}
