use crate::utils::{to_num, to_u_num};
use good_lp::{
    coin_cbc, constraint, default_solver, solvers::coin_cbc, variable, variables, Solution,
    SolverModel, Variable,
};
use std::{collections::HashSet, fs};

fn state_to_bits(state: &str) -> u32 {
    state.chars().rev().enumerate().fold(
        0,
        |acc, (i, ch)| {
            if ch == '#' {
                acc | (1 << i)
            } else {
                acc
            }
        },
    )
}

fn button_to_bits(buttons: &str, len: usize) -> Vec<u32> {
    buttons
        .split(" ")
        .map(|button_str| {
            let bits_end = button_str.find(")").unwrap();
            button_str[1..bits_end]
                .split(",")
                .map(to_num)
                .fold(0, |acc, num| acc | (1 << (len - num as usize - 1)))
        })
        .collect()
}

fn traverse(state: u32, buttons: &Vec<u32>, presses: &HashSet<u32>, t_min: usize) -> usize {
    if state == 0 {
        return presses.len();
    }

    if presses.len() > t_min {
        return t_min;
    }

    let mut min = t_min;
    for button in buttons {
        if presses.contains(button) {
            continue;
        }

        let mut new_presses = (*presses).clone();
        new_presses.insert(*button);
        let num_presses = traverse(state ^ button, buttons, &new_presses, min);
        if num_presses < min {
            min = num_presses;
        }
    }

    min
}

fn solve_part_1() {
    let contents = fs::read_to_string("./src/input.txt").expect("Should read the file");
    let answer = contents.lines().fold(0, |acc, line| {
        let final_state_end = line.find("]").unwrap();
        let buttons_end = line.find("{").unwrap();
        let final_state = &line[1..final_state_end];
        let buttons = &line[final_state_end + 1..buttons_end].trim();
        let state_bits = state_to_bits(final_state);
        let button_bits = button_to_bits(buttons, final_state.len());
        let min_presses = traverse(state_bits, &button_bits, &HashSet::new(), usize::MAX);
        acc + min_presses
    });

    println!("Part 1 Answer: {answer} \n");
}

fn solve_system(buttons: &Vec<Vec<usize>>, joltages: &Vec<f64>) -> f64 {
    let mut vars = variables!();
    let press_counts: Vec<Variable> = (0..buttons.len())
        .map(|_| vars.add(variable().min(0)))
        .collect();

    let objective = press_counts.iter().sum::<good_lp::Expression>();

    let mut model = vars.minimise(objective).using(default_solver);

    for (target_counter, &joltage) in joltages.iter().enumerate() {
        let mut equation = good_lp::Expression::from(0);
        for (btn_idx, affected_counters) in buttons.iter().enumerate() {
            if affected_counters.contains(&target_counter) {
                equation += press_counts[btn_idx];
            }
        }
        model.add_constraint(constraint!(equation == joltage));
    }

    match model.solve() {
        Ok(solution) => solution
            .eval(press_counts.iter().sum::<good_lp::Expression>())
            .round(),
        Err(e) => {
            println!("\n\n\n");
            println!("Error: {e}");
            println!("\n\n\n");
            -1.0
        }
    }
}

fn solve_part_2() {
    let contents = fs::read_to_string("./src/input.txt").expect("Should read the file");
    let answer = contents.lines().fold(0.0, |acc, line| {
        let final_state_end = line.find("]").unwrap();
        let buttons_end = line.find("{").unwrap();
        let joltages_end = line.find("}").unwrap();
        let buttons_str = &line[final_state_end + 1..buttons_end].trim();
        let joltages_str = &line[buttons_end + 1..joltages_end].trim();
        let buttons: Vec<Vec<usize>> = buttons_str
            .split(" ")
            .map(|button_str| {
                let bits_end = button_str.find(")").unwrap();
                button_str[1..bits_end].split(",").map(to_u_num).collect()
            })
            .collect();

        let joltages: Vec<f64> = joltages_str
            .split(",")
            .map(|n| n.parse().unwrap())
            .collect();
        let min_presses = solve_system(&buttons, &joltages);
        if min_presses == -1.0 {
            println!("-------------");
        }

        acc + min_presses
    });

    println!("Part 2 Answer: {:.0} \n", answer);
}

pub fn solve() {
    // self::solve_part_1();
    self::solve_part_2();
}
