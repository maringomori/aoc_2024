
use std::collections::HashMap;

pub fn run1(input: &[String]) {
    let mut col1: Vec<i32> = Vec::new();
    let mut col2: Vec<i32> = Vec::new();

    for line in input {
        let columns: Vec<&str> = line.split_whitespace().collect();
        if let (Ok(val1), Ok(val2)) = (columns[0].parse::<i32>(), columns[1].parse::<i32>()) {
            col1.push(val1);
            col2.push(val2);
        }
    }

    col1.sort();
    col2.sort();

    let mut diff = 0;
    for i in 0..col1.len() {
        diff +=  (col1[i] - col2[i]).abs();
    }

    print!("Distance: {}", diff);
}

pub fn run2(input: &[String]) {
    let mut col1: Vec<i32> = Vec::new();
    let mut col2: HashMap<i32, i32> = HashMap::new();

    for line in input {
        let columns: Vec<&str> = line.split_whitespace().collect();
        if let Ok(val1 ) = columns[0].parse::<i32>() {
            col1.push(val1);
        }
     
        if let Ok(val2) = columns[1].parse::<i32>() {
            *col2.entry(val2).or_insert(0) += 1;
        } 
    }

    let mut sim_score = 0;
    for key in col1 {
        if let Some(count2) = col2.get(&key) {
            sim_score += key * count2;
        }
    }

    println!("sim_score: {}", sim_score);
}