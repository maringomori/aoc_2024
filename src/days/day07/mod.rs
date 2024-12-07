
fn solve(key: i128, values: &[i128], i: usize, curr: i128) -> bool {
    if i == values.len() {
        return curr == key;
    }

    if solve(key, values, i + 1, curr + values[i]) {
        return true;
    }

    solve(key, values, i + 1, curr * values[i]) 
}

fn solve2(key: i128, values: &[i128], i: usize, curr: i128) -> bool {
    if i == values.len() {
        return curr == key;
    }

    if solve2(key, values, i + 1, curr + values[i]) {
        return true;
    }

    if solve2(key, values, i + 1, curr * values[i]) {
        return true;
    }

    solve2(key, values, i + 1, format!("{}{}", curr, values[i]).parse().unwrap())
}

pub fn run1(input: &[String]) {
    let mut sum: i128 = 0;

    print!("Line count: {}", input.len());
    for line in input {
        let parts: Vec<&str> = line.split(':').collect();
        let key: i128 = parts[0].trim().parse().unwrap();
        let values: Vec<i128> = parts[1]
            .split_whitespace()
            .map(|s| s.trim().parse().unwrap())
            .collect::<Vec<_>>();

        if solve(key, &values, 1, values[0]) {
            sum += key;
        }
    }

    println!("Sum: {}", sum);
}

pub fn run2(input: &[String]) {
    let mut sum: i128 = 0;

    print!("Line count: {}", input.len());
    for line in input {
        let parts: Vec<&str> = line.split(':').collect();
        let key: i128 = parts[0].trim().parse().unwrap();
        let values: Vec<i128> = parts[1]
            .split_whitespace()
            .map(|s| s.trim().parse().unwrap())
            .collect::<Vec<_>>();

        if solve2(key, &values, 1, values[0]) {
            sum += key;
        }
    }

    println!("Sum: {}", sum);
}