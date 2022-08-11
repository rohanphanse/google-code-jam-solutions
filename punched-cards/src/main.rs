use std::io::{ self, BufRead };
use std::fs;

fn main() {
    // let lines: Vec<String> = io::stdin().lock().lines().map(|x| x.unwrap()).collect();
    let contents: String = fs::read_to_string("./data/input.txt").unwrap();
    let lines: Vec<String> = contents.split("\n").map(|s| s.to_string()).collect();
    let mut dimensions: Vec<(i32, i32)> = Vec::with_capacity(lines.len() - 1);
    for i in 1..lines.len() {
        let strings: Vec<&str> = lines[i].split(" ").collect();
        let numbers: Vec<i32> = strings.iter().map(|s| s.parse::<i32>().unwrap()).collect();
        dimensions.push((numbers[0], numbers[1]));
    }
    for i in 0..dimensions.len() {
        println!("Case #{}:", i + 1);
        let (rows, columns) = dimensions[i];
        print_card(rows, columns);
    }
}

fn print_card(rows: i32, columns: i32) {
    for r in 0..(2 * rows + 1) {
        let symbol = if r % 2 == 0 { "+" } else { "|" };
        let spacer = if r % 2 == 0 { "-" } else { "." };
        for c in 0..(2 * columns + 1) {
            if r <= 1 && c <= 1 {
                print!(".");
            } else if c % 2 == 0 {
                print!("{}", symbol);
            } else {
                print!("{}", spacer);
            }
        }
        println!();
    }
}
