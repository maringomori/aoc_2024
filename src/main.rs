use chrono::Datelike;
use chrono::Local;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
mod days;

fn read_input(file_path: &str) -> io::Result<Vec<String>> {
    let path = Path::new(file_path);
    let file = File::open(&path)?;
    let reader = io::BufReader::new(file);
    reader.lines().collect()
}


fn main() {
    let today = Local::now();
    let day = today.day();

    match day {
        1 => {
            let input = read_input("src/days/day01/input02.txt").expect("Failed to read input");
            days::day01::run1(&input);
            days::day01::run2(&input);
        },
        _ => println!("No challenge for this day!"),
    }
}