use std::io::{ self, BufRead };
use std::fs;

fn main() {
    // let lines: Vec<String> = io::stdin().lock().lines().map(|x| x.unwrap()).collect();
    let contents: String = fs::read_to_string("./data/input.txt").unwrap();
    let lines: Vec<String> = contents.split("\n").map(|s| s.to_string()).collect();
    let mut cases: Vec<Vec<Vec<i32>>> = Vec::with_capacity((lines.len() - 1) / 3);
    let mut i = 1;
    while i < lines.len() {
        let mut printers: Vec<Vec<i32>> = Vec::with_capacity(3);
        for j in i..(i + 3) {
            let strings: Vec<&str> = lines[j].split(" ").collect();
            let numbers: Vec<i32> = strings.iter().map(|s| s.parse::<i32>().unwrap()).collect();
            printers.push(numbers);
        }
        cases.push(printers);
        i += 3;
    }
    for i in 0..cases.len() {
        print!("Case #{}: ", i + 1);
        print_solution(&cases[i]);
    }
}

fn print_solution(printers: &Vec<Vec<i32>>) {
    let mut solution: Vec<i32> = vec![0; printers[0].len()];
    for c in 0..printers[0].len() {
        let mut minimum = printers[0][c];
        for r in 1..printers.len() {
            if printers[r][c] < minimum {
                minimum = printers[r][c];
            }
        }
        solution[c] = minimum;
    }
    let sum = solution.iter().sum::<i32>();
    if sum < 1_000_000 {
        println!("IMPOSSIBLE");
        return;
    }
    let mut extra = sum - 1_000_000;
    for i in 0..solution.len() {
        if extra > solution[i] {
            extra -= solution[i];
            solution[i] = 0;
        } else {
            solution[i] -= extra;
            extra = 0;
        }
    }
    let solution_string: String = solution.iter().map(|n| n.to_string()).collect::<Vec<String>>().join(" ");
    println!("{}", solution_string);
}
