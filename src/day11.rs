use std::collections::HashMap;
use std::fs;

fn count_paths<'a>(
    graph: &HashMap<&'a str, Vec<&'a str>>,
    current_node: &'a str,
    target_node: &'a str,
    memo: &mut HashMap<&'a str, usize>,
) -> usize {
    if current_node == target_node {
        return 1;
    }

    if let Some(&count) = memo.get(current_node) {
        return count;
    }

    let result = graph
        .get(current_node)
        .map(|vs| {
            vs.iter()
                .map(|next_node| count_paths(graph, next_node, target_node, memo))
                .sum()
        })
        .unwrap_or(0);

    memo.insert(current_node, result);
    result
}

fn get_segment_count<'a>(
    graph: &HashMap<&'a str, Vec<&'a str>>,
    start: &'a str,
    end: &'a str,
) -> usize {
    let mut memo = HashMap::new();
    count_paths(graph, start, end, &mut memo)
}

fn solve_part_1() {
    let contents = fs::read_to_string("./src/input.txt").expect("Should read the file");
    let mut connections = HashMap::new();

    contents.lines().for_each(|line| {
        let parts: Vec<&str> = line.split(':').collect();

        let source = parts[0].trim();
        let dests: Vec<&str> = parts[1].trim().split_whitespace().collect();

        connections
            .entry(source)
            .or_insert_with(Vec::new)
            .extend(dests);
    });

    let answer = get_segment_count(&connections, "you", "out");

    println!("Part 1 Answer: {answer}");
}

fn solve_part_2() {
    let contents = fs::read_to_string("./src/input.txt").expect("Should read the file");
    let mut connections = HashMap::new();

    contents.lines().for_each(|line| {
        let parts: Vec<&str> = line.split(':').collect();

        let source = parts[0].trim();
        let dests: Vec<&str> = parts[1].trim().split_whitespace().collect();

        connections
            .entry(source)
            .or_insert_with(Vec::new)
            .extend(dests);
    });

    let path_1: usize = [
        get_segment_count(&connections, "svr", "dac"),
        get_segment_count(&connections, "dac", "fft"),
        get_segment_count(&connections, "fft", "out"),
    ]
    .iter()
    .product();

    let path_2: usize = [
        get_segment_count(&connections, "svr", "fft"),
        get_segment_count(&connections, "fft", "dac"),
        get_segment_count(&connections, "dac", "out"),
    ]
    .iter()
    .product();

    let answer = path_1 + path_2;

    println!("Part 2 Answer: {answer}");
}

pub fn solve() {
    solve_part_1();
    solve_part_2();
}
