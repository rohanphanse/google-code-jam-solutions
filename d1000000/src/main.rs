use std::io::{ self, BufRead };
use std::fs;

fn main() {
    // let lines: Vec<String> = io::stdin().lock().lines().map(|x| x.unwrap()).collect();
    let contents: String = fs::read_to_string("./data/input.txt").unwrap();
    let lines: Vec<String> = contents.split("\n").map(|s| s.to_string()).collect();
    let mut cases: Vec<Vec<i32>> = Vec::new();
    for i in 1..lines.len() {
        if i % 2 == 1 { continue };
        let strings: Vec<&str> = lines[i].split(" ").collect();
        let mut numbers: Vec<i32> = strings.iter().map(|s| s.parse::<i32>().unwrap()).collect();
        numbers.sort();
        cases.push(numbers);
    }
    for i in 0..cases.len() {
        println!("Case #{}: {}", i + 1, find_longest_straight(&cases[i]));
        
    }
}

fn find_longest_straight(dice: &Vec<i32>) -> i32 {
    let mut straight = 0;
    for d in dice {
        if *d > straight {
            straight += 1;
        }
    }
    straight
}