use regex::Regex;

fn extract_mul_expressions(input: &str) -> Vec<String> {
    let re = Regex::new(r"mul\(\d+,\d+\)").unwrap();
    re.find_iter(input).map(|mat| mat.as_str().to_string()).collect()
}

fn extract_expressions2(input: &str) -> Vec<String> {
    let re = Regex::new(r"\b(do|don't)\b|mul\(\d+,\d+\)").unwrap();
    re.find_iter(input).map(|mat| mat.as_str().to_string()).collect()
}

fn multiply_and_sum(expressions: &[String]) -> i32 {
    let re = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();
    let mut sum = 0;

    for expr in expressions {
        if let Some(caps) = re.captures(expr) {
            let num1: i32 = caps[1].parse().unwrap();
            let num2: i32 = caps[2].parse().unwrap();
            sum += num1 * num2;
        }
    }

    sum
}

fn multiply_and_sum2(expressions: &[String]) -> i32 {
    let re = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();
    let mut sum = 0;
    let mut skip_next_mul = false;

    for expr in expressions {
        if expr == "don't" {
            skip_next_mul = true;
        } else if expr == "do" {
            skip_next_mul = false;
        } else if let Some(caps) = re.captures(expr) {
            if skip_next_mul {
                continue;
            }
            let num1: i32 = caps[1].parse().unwrap();
            let num2: i32 = caps[2].parse().unwrap();
            sum += num1 * num2;
        }
    }

    sum
}

pub fn run1(input: &[String]) {
    let mut all_mul_expressions: Vec<String> = Vec::new();

    for line in input {
        let mul_expressions = extract_mul_expressions(line);
        all_mul_expressions.extend(mul_expressions);
    }

    let result = multiply_and_sum(&all_mul_expressions);
    println!("{}", result);

}


pub fn run2(input: &[String]) {
    let mut all_mul_expressions: Vec<String> = Vec::new();

    for line in input {
        let mul_expressions = extract_expressions2(line);
        all_mul_expressions.extend(mul_expressions);
    }

    let result = multiply_and_sum2(&all_mul_expressions);
    println!("{}", result);
}


