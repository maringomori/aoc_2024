use std::collections::HashMap;

fn check_diagonal(input: &[String], row: usize, col: usize, word: &str) -> usize {
    let n = input.len();
    let word_len = word.len();
    let mut count = 0;

    // tleft bright
    if row + word_len <= n && col + word_len <= n {
        let mut diagonal_word = String::new();
        for k in 0..word_len {
            diagonal_word.push(input[row + k].chars().nth(col + k).unwrap_or(' '));
        }
        if diagonal_word == word {
            count += 1;
        }
    }

    // tright bleft 
    if row + 1 >= word_len && col + word_len <= n {
        let mut diagonal_word = String::new();
        for k in 0..word_len {
            diagonal_word.push(input[row - k].chars().nth(col + k).unwrap_or(' '));
        }
        if diagonal_word == word {
            count += 1;
        }
    }

    count
}


fn check_diagonal2(input: &[String], row: usize, col: usize, word: &str, a_positions: &mut Vec<(usize, usize)>) -> usize {
    let n = input.len();
    let word_len = word.len();
    let mut count = 0;
  
    // tleft bright
    if row + word_len <= n && col + word_len <= n {
        let mut diagonal_word = String::new();
        for k in 0..word_len {
            diagonal_word.push(input[row + k].chars().nth(col + k).unwrap_or(' '));
        }
        if diagonal_word == word {
            count += 1;
            a_positions.push((row + 1, col + 1));
        }
    }

    // tright bleft 
    if row + 1 >= word_len && col + word_len <= n {
        let mut diagonal_word = String::new();
        for k in 0..word_len {
            diagonal_word.push(input[row - k].chars().nth(col + k).unwrap_or(' '));
        }
        if diagonal_word == word {
            count += 1;
            a_positions.push((row - 1, col + 1));
        }
    }

    count
}

fn count_occurrences(line: &str, word: &str) -> usize {
    line.match_indices(word).count()
}

pub fn run1(input: &[String]) {
    let word = "XMAS";
    let mut count = 0;

    for (i, line) in input.iter().enumerate() {
        print!("{}\n", line);
        count += count_occurrences(line, word);
        count += count_occurrences(line, &word.chars().rev().collect::<String>());

        let vertical_word: String = input.iter().map(|s| s.chars().nth(i).unwrap_or(' ')).collect();
        count += count_occurrences(&vertical_word, word);
        count += count_occurrences(&vertical_word, &word.chars().rev().collect::<String>());
    }

    let n = input.len();
    for i in 0..n {
        for j in 0..n {
            count += check_diagonal(input, i, j, word);
            count += check_diagonal(input, i, j, &word.chars().rev().collect::<String>());
        }
    }

    println!("{}", count);
}

pub fn run2(input: &[String]) {
    let word = "MAS";
    let mut _count = 0;
    
    let mut a_positions: Vec<(usize, usize)> = Vec::new();

    let n = input.len();
    for i in 0..n {
        for j in 0..n {
            _count += check_diagonal2(input, i, j, word, &mut a_positions);
            _count += check_diagonal2(input, i, j, &word.chars().rev().collect::<String>(), &mut a_positions);
        }
    }

    let mut a_count = 0;
    let mut a_map = HashMap::new();
    for pos in a_positions {
        *a_map.entry(pos).or_insert(0) += 1;
    }
    for &_count in a_map.values() {
        if _count > 1 {
            a_count += 1;
        }
    }
    println!("{}", a_count);
}