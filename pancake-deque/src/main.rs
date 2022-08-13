// use std::io::{ self, BufRead };
use std::fs;

fn main() {
    // let lines: Vec<String> = io::stdin().lock().lines().map(|x| x.unwrap()).collect();
    let contents: String = fs::read_to_string("./data/input.txt").unwrap();
    let lines: Vec<String> = contents.split("\n").map(|s| s.to_string()).collect();
    let mut cases: Vec<Vec<i32>> = Vec::new();
    for i in 0..((lines.len() - 1) / 2) {
        let strings: Vec<&str> = lines[i * 2 + 2].split(" ").collect();
        let numbers: Vec<i32> = strings.iter().map(|s| s.parse::<i32>().unwrap()).collect();
        cases.push(numbers);
    }
    for i in 0..cases.len() {
        println!("Case #{}: {}", i + 1, maximize_pancake_payers(&cases[i]));
    }
}

fn maximize_pancake_payers(pancakes_ref: &Vec<i32>) -> i32 {
    let mut pancakes = pancakes_ref.clone();
    let mut max = 0;
    let mut payers = 0;
    for _ in 0..(pancakes.len() - 1) {
        let first = pancakes[0];
        let last = pancakes[pancakes.len() -  1];
        // Remove minimum, pancake with lesser deliciousness level
        let removed_index = if i32::min(first, last) == first { 0 } else { pancakes.len() - 1 };
        if pancakes[removed_index] >= max {
            max = pancakes[removed_index];
            payers += 1;
        }
        pancakes.remove(removed_index);
    }
    // One pancake left
    if pancakes[0] >= max {
        payers += 1;
    }
    payers
}