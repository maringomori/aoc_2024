pub fn check_order(pair: &(i32, i32), array: &[i32]) -> bool {
    let first = pair.0;
    let second = pair.1;

    let first_index = array.iter().position(|&x| x == first);
    let second_index = array.iter().position(|&x| x == second);


    match (first_index, second_index) {
        (Some(first_index), Some(second_index)) => {
            if first_index < second_index {
                return true;
            } else {
                return false;
            }
        },
        _ => return true,
    }
}

pub fn find_middle_index_value(array: &[i32]) -> i32 {
    let middle_index = array.len() / 2;
    let middle_value = array[middle_index];
    middle_value
}

pub fn correct_order(array: &mut Vec<i32>, pair: &(i32, i32)) {
    let first = pair.0;
    let second = pair.1;

    let first_index = array.iter().position(|&x| x == first);
    let second_index = array.iter().position(|&x| x == second);

    if let (Some(first_index), Some(second_index)) = (first_index, second_index) {
        if first_index > second_index {
            array.swap(first_index, second_index);
        }
    }
}

pub fn run1(input: &[String]) {
    let mut first_array: Vec<(i32, i32)> = Vec::new();
    let mut second_array: Vec<Vec<i32>> = Vec::new();
    let mut is_first_array = true;

    for line in input {
        if line.trim().is_empty() {
            is_first_array = false;
            continue;
        }

        if is_first_array {
            let parts: Vec<&str> = line.split('|').collect();
            if parts.len() == 2 {
                if let (Ok(first), Ok(second)) = (parts[0].parse::<i32>(), parts[1].parse::<i32>()) {
                    first_array.push((first, second));
                }
            }
        } else {
            let numbers: Vec<i32> = line.split(',')
                .filter_map(|s| s.trim().parse::<i32>().ok())
                .collect();
            second_array.push(numbers);
        }
    }

    for pair in &first_array {
        for array in &second_array {
            check_order(pair, array);
        }
    }
    
    let mut sum = 0;
    let mut sum_wrong_order = 0;
    for array in &second_array {
        let mut all_true = true;
        for pair in &first_array {
            if !check_order(pair, array) {
                all_true = false;
                break;
            }
        }
        if all_true {
            sum += find_middle_index_value(array);
        } else {
            let mut array_clone = array.clone();
            // please dont blow up (worst code YET)
            while !first_array.iter().all(|pair| check_order(pair, &array_clone)) {
                for pair in &first_array {
                    correct_order(&mut array_clone, pair);
                }
            }

            let mid = find_middle_index_value(&array_clone);
            sum_wrong_order += mid;
        }

    }

    println!("sum: {}", sum);
    println!("sum wrong: {}", sum_wrong_order);
}