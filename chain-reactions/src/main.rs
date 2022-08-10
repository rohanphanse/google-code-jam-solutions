use std::io::{ self, BufRead };
use std::i64;
use std::fs;

fn main() {
    let cases = get_cases();
    let results = get_results();
    let mut wrong = 0;
    for i in 0..cases.len() {
        let (fun_factors, pointers) = &cases[i];
        // println!("Case #{}: {}", i + 1, calculate_maximum_fun(fun_factors, pointers));
        let result = format!("Case #{}: {}", i + 1, calculate_maximum_fun(fun_factors, pointers));
        if result != results[i] {
            println!("Expected {}, got {}", &results[i], result);
            wrong += 1;
        }
    }
    println!("{}/{} cases passed", cases.len() - wrong, cases.len());
}

fn get_cases() -> Vec<(Vec<i64>, Vec<usize>)> {
    // let lines: Vec<String> = io::stdin().lock().lines().map(|x| x.unwrap()).collect();
    let contents: String = fs::read_to_string("./data/ts1_input.txt").unwrap();
    let lines: Vec<String> = contents.split("\n").map(|s| s.to_string()).collect();
    let mut cases: Vec<(Vec<i64>, Vec<usize>)> = Vec::new();
    for i in 0..((lines.len() - 1) / 3) {
        let fun_factor_strings: Vec<&str> = lines[i * 3 + 2].split(" ").collect();
        let fun_factors: Vec<i64> = fun_factor_strings.iter().map(|s| s.parse::<i64>().unwrap()).collect();
        let pointer_strings: Vec<&str> = lines[i * 3 + 3].split(" ").collect();
        let pointers: Vec<usize> = pointer_strings.iter().map(|s| s.parse::<usize>().unwrap()).collect();
        cases.push((fun_factors, pointers));
    }
    cases
}

fn get_results() -> Vec<String> {
    let contents: String = fs::read_to_string("./data/ts1_output.txt").unwrap();
    let lines: Vec<String> = contents.split("\n").map(|s| s.to_string()).collect();
    lines
}

fn calculate_maximum_fun(fun_factors: &Vec<i64>, pointers: &Vec<usize>) -> i64 {
    let mut trees: Vec<Vec<(i64, usize)>> = vec![Vec::new(); fun_factors.len() + 1];
    for i in 0..fun_factors.len() {
        trees[pointers[i]].push((fun_factors[i], i + 1));
    }
    // println!("Trees: {:?}", &trees);
    let mut fun = 0;
    for i in 0..trees[0].len() {
        fun += recursively_calculate_maximum_fun(&trees, (0, i), &mut fun);
    }
    fun
}

fn recursively_calculate_maximum_fun(trees: &Vec<Vec<(i64, usize)>>, node_location: (usize, usize), global_fun: &mut i64) -> i64 {
    let (fun_factor, children_pointer) = trees[node_location.0][node_location.1];
    let children: &Vec<(i64, usize)> = &trees[children_pointer];
    if children.len() == 0 {
        return fun_factor;
    } else if children.len() == 1 {
        return i64::max(fun_factor, recursively_calculate_maximum_fun(trees, (children_pointer, 0), global_fun));
    } else {
        let mut min_fun: i64 = i64::MAX;
        let mut total_fun: i64 = 0;
        for c in 0..children.len() {
            let fun = recursively_calculate_maximum_fun(trees, (children_pointer, c), global_fun);
            if fun < min_fun {
                min_fun = fun;
            }
            total_fun += fun;
        }
        *global_fun += total_fun - min_fun;
        return i64::max(fun_factor, min_fun);
    }
}



