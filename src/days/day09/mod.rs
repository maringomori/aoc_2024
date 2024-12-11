
fn swap_numbers_with_dots(uncompressed: Vec<i32>) -> Vec<i32> {
    let mut chars = uncompressed;
    let mut left = 0;
    let mut right = chars.len() - 1;

    while left < right {
        while left < right && chars[left] != -1 {
            left += 1;
        }
        while left < right && chars[right] == -1 {
            right -= 1;
        }
        if left < right {
            chars.swap(left, right);
            left += 1;
            right -= 1;
        }
    }

    chars
}

fn swap_numbers_with_dots2(uncompressed: Vec<i32>) -> Vec<i32> {
    let mut chars = uncompressed;
    let mut file_positions: Vec<(i32, usize, usize)> = Vec::new();

    let mut i = 0;
    while i < chars.len() {
        if chars[i] != -1 {
            let file_id = chars[i];
            let start = i;
            while i < chars.len() && chars[i] == file_id {
                i += 1;
            }
            let end = i;
            file_positions.push((file_id, start, end));
        } else {
            i += 1;
        }
    }

    println!("file_positions: {:?}", file_positions);
    println!("chars: {:?}", chars);
    file_positions.sort_by(|a, b| b.0.cmp(&a.0));

    for (file_id, start, end) in file_positions {
        let file_length = end - start;
        let mut leftmost_free_space = None;

        let mut free_space_start = None;
        let mut free_space_length = 0;
        for i in 0..start {
            if chars[i] == -1 {
                if free_space_start.is_none() {
                    free_space_start = Some(i);
                }
                free_space_length += 1;
                if free_space_length >= file_length {
                    leftmost_free_space = free_space_start;
                    break;
                }
            } else {
                free_space_start = None;
                free_space_length = 0;
            }
        }

        if let Some(free_start) = leftmost_free_space {
            for i in 0..file_length {
                chars[free_start + i] = file_id;
            }

            for i in start..end {
                chars[i] = -1;
            }
        }
    }

    chars
}

fn calculate_checksum(result: Vec<i32>) -> i64 {
    let mut id: i64= 0; 
    let mut sum: i64 = 0;
    for i in 0..result.len() {
        if result[i] != -1 {
            let char_num = result[i];
            let prod = char_num as i64 * id;
            sum += prod;
        }       
        id += 1;
    }

    sum as i64
}

pub fn run1(input: &[String]) {
 

    // files, free 
    // növekvő id
    // jobb oldali számot balra mozgatni . helyére

    let input_chars: Vec<Vec<char>> = input.iter().map(|line| line.chars().collect()).collect();

    println!("input_chars: {:?}", input_chars);
    let mut uncompressed: Vec<i32> = Vec::new();

    let mut counter = 0;
    let mut id = 0;
    for char in input_chars.first().unwrap() {
        if counter % 2 == 0 {
            for _ in 0..char.to_digit(10).unwrap() {
                uncompressed.push(id);
            }
            id += 1;
        } else {
            for _ in 0..char.to_digit(10).unwrap() {
                uncompressed.push(-1);
            }
        }
        counter += 1;
    }

    println!("uncompressed: {:?}", uncompressed);
    let result = swap_numbers_with_dots(uncompressed);

    println!("final_result: {:?}", result);

    let checksum = calculate_checksum(result);
    println!("Checksum: {}", checksum);

}



pub fn run2(input: &[String]) {
    let input_chars: Vec<Vec<char>> = input.iter().map(|line| line.chars().collect()).collect();

    println!("input_chars: {:?}", input_chars);
    let mut uncompressed: Vec<i32> = Vec::new();

    let mut counter = 0;
    let mut id = 0;
    for char in input_chars.first().unwrap() {
        if counter % 2 == 0 {
            for _ in 0..char.to_digit(10).unwrap() {
                uncompressed.push(id);
            }
            id += 1;
        } else {
            for _ in 0..char.to_digit(10).unwrap() {
                uncompressed.push(-1);
            }
        }
        counter += 1;
    }

    println!("uncompressed: {:?}", uncompressed);
    let result = swap_numbers_with_dots2(uncompressed);

    println!("final_result: {:?}", result);
    println!("final result as string: {:?}", result.iter().map(|x| x.to_string()).collect::<String>());


    let checksum = calculate_checksum(result);
    println!("Checksum: {}", checksum);
}
