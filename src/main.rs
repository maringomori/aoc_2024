use chrono::Datelike;
use chrono::Local;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::time::Instant;
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
            let input = read_input("src/days/day01/input01.txt").expect("Failed to read input");
            let start = Instant::now();
            days::day01::run1(&input);
            let duration = start.elapsed();
            println!("run1 took: {:?}", duration);

            let input = read_input("src/days/day01/input02.txt").expect("Failed to read input");
            let start = Instant::now();
            days::day01::run2(&input);
            let duration = start.elapsed();
            println!("run2 took: {:?}", duration);
        },
        2 => {
            let input = read_input("src/days/day02/input01.txt").expect("Failed to read input");
            
            let start = Instant::now();
            days::day02::run1(&input);
            let duration = start.elapsed();
            println!("run1 took: {:?}", duration);

            let input = read_input("src/days/day02/input02.txt").expect("Failed to read input");
            let start = Instant::now();
            days::day02::run2(&input);
            let duration = start.elapsed();
            println!("run2 took: {:?}", duration);
        },
        3 => {
          
            let input = read_input("src/days/day03/input01.txt").expect("Failed to read input");
            
            let start = Instant::now();
            days::day03::run1(&input);
            let duration = start.elapsed();
            println!("run1 took: {:?}", duration);
    

            let input = read_input("src/days/day03/input02.txt").expect("Failed to read input");
            let start = Instant::now();
            days::day03::run2(&input);
            let duration = start.elapsed();
            println!("run2 took: {:?}", duration);
        },
        4 => {
           

            let input = read_input("src/days/day04/input01.txt").expect("Failed to read input");
            
            let start = Instant::now();
            days::day04::run1(&input);
            let duration = start.elapsed();
            println!("run1 took: {:?}", duration);
           
            let input = read_input("src/days/day04/input02.txt").expect("Failed to read input");
            let start = Instant::now();
            days::day04::run2(&input);
            let duration = start.elapsed();
            println!("run2 took: {:?}", duration);
        },
        5 => {
            
            let input = read_input("src/days/day05/input01.txt").expect("Failed to read input");
            
            let start = Instant::now();
            days::day05::run1(&input);
            let duration = start.elapsed();
            println!("run1 took: {:?}", duration);
        },
        _ => println!("No challenge for this day!"),
    }
}