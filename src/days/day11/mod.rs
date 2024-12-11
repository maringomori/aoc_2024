use std::collections::HashMap;

pub fn blink(numbers: &[i64]) -> Vec<i64> {
    let mut new_numbers: Vec<i64> = Vec::new();
     
    for i in 0..numbers.len() {
        if numbers[i] == 0 {
            new_numbers.push(1);
            continue;
            
        }
        if numbers[i].to_string().len() % 2 == 0 {
            let str_num = numbers[i].to_string().len() / 2;
            let num_str = numbers[i].to_string();
            let (first_half, second_half) = num_str.split_at(str_num);
            new_numbers.push(first_half.parse::<i64>().unwrap());
            new_numbers.push(second_half.parse::<i64>().unwrap());
        } else {
            new_numbers.push(numbers[i] * 2024);
        }
    }

    new_numbers
}

pub fn blink2(number_counts: &HashMap<i64, usize>) -> HashMap<i64, usize> {
    let mut new_number_counts: HashMap<i64, usize> = HashMap::new();

    for (&number, &count) in number_counts {
        if number == 0 {
            *new_number_counts.entry(1).or_insert(0) += count;
        } else {
            let num_str = number.to_string();
            let len = num_str.len();
            if len % 2 == 0 {
                let str_num = len / 2;
                let (first_half, second_half) = num_str.split_at(str_num);
                let first_half_num = first_half.parse::<i64>().unwrap();
                let second_half_num = second_half.parse::<i64>().unwrap();
                *new_number_counts.entry(first_half_num).or_insert(0) += count;
                *new_number_counts.entry(second_half_num).or_insert(0) += count;
            } else {
                *new_number_counts.entry(number * 2024).or_insert(0) += count;
            }
        }
    }

    new_number_counts
}

pub fn run1(input: &[String]) {
    let first_row = &input[0];
    let mut numbers: Vec<i64> = first_row.split_whitespace().filter_map(|s| s.trim().parse().ok()).collect();

    for _ in 0..25 {
        numbers = blink(&numbers);
    }
    
    println!("{}", numbers.len());
}


pub fn run2(input: &[String]) {
    let first_row = &input[0];
    let mut number_counts: HashMap<i64, usize> = HashMap::new();

    for number_str in first_row.split_whitespace() {
        let number = number_str.trim().parse::<i64>().unwrap();
        *number_counts.entry(number).or_insert(0) += 1;
    }

    for _ in 0..75 {
        number_counts = blink2(&number_counts);
    }
    
    let total_count: usize = number_counts.values().sum();
    println!("{}", total_count);
}

