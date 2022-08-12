// use std::io::{ self, BufRead };
use std::fs;

fn main() {
    // let lines: Vec<String> = io::stdin().lock().lines().map(|x| x.unwrap()).collect();
    let contents: String = fs::read_to_string("./data/input.txt").unwrap();
    let lines: Vec<String> = contents.split("\n").map(|s| s.to_string()).collect();
    for i in 1..lines.len() {
        println!("Case #{}: {}", i, first_highlighted_string(&lines[i]));
    }
}

fn first_highlighted_string(input: &String) -> String {
    let chars: Vec<char> = input.chars().collect();
    let mut new_chars: Vec<char> = Vec::new();
    let empty_index = chars.len();
    let mut equals_index = empty_index;
    for i in 0..(chars.len() - 1) {
        if chars[i] == chars[i + 1] && equals_index == empty_index {
            equals_index = i;
        }
        if chars[i] < chars[i + 1] {
            if equals_index != empty_index {
                for j in equals_index..i {
                    new_chars.push(chars[equals_index]);
                }
            }
            new_chars.push(chars[i]);
        }
        if chars[i] != chars[i + 1] {
            equals_index = empty_index;
        }
        new_chars.push(chars[i]);
    }
    new_chars.push(chars[chars.len() - 1]); // Always add last character once
    let string = new_chars.iter().collect();
    string
}