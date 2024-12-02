
fn is_safe(arr: &[i32]) -> bool {
    let mut increasing = true;
    let mut decreasing = true;

    for i in 1..arr.len() {
        let diff = (arr[i] - arr[i - 1]).abs();
        if diff < 1 || diff > 3 {
            return false;
        }
        if arr[i] < arr[i - 1] {
            increasing = false;
        }
        if arr[i] > arr[i - 1] {
            decreasing = false;
        }
    }

    increasing || decreasing
}


fn is_safe_with_removal(arr: &[i32]) -> bool {
    if is_safe(arr) {
        return true;
    }

    for i in 0..arr.len() {
        let mut new_arr = arr.to_vec();
        new_arr.remove(i);
        if is_safe(&new_arr) {
            return true;
        }
    }

    return false;
}

pub fn run1(input: &[String]) {
    let mut arrs: Vec<Vec<i32>> = Vec::new();
    let mut safe_count = 0;

    for line in input {
        let i: Vec<i32> = line
            .split_whitespace()
            .filter_map(|s| s.parse::<i32>().ok())
            .collect();
        arrs.push(i);
    }

    for arr in &arrs {
        if is_safe(arr) {
            safe_count += 1;
        }
    }


    println!("Safe count: {}", safe_count);
}

pub fn run2(input: &[String]) {
    let mut arrs: Vec<Vec<i32>> = Vec::new();
    let mut safe_count = 0;

    for line in input {
        let i: Vec<i32> = line
            .split_whitespace()
            .filter_map(|s| s.parse::<i32>().ok())
            .collect();
        arrs.push(i);
    }

    for arr in &arrs {
        if is_safe_with_removal(arr) {
            safe_count += 1;
        }
    }

    println!("Safe count: {}", safe_count);
}

