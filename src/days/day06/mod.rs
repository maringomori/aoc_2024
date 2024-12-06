use std::collections::HashSet;

pub fn run1(input: &[String]) {
    let mut grid: Vec<Vec<char>> = input.iter().map(|line| line.chars().collect()).collect();
    let mut caret_location = None;

    for (row, line) in grid.iter().enumerate() {
        for (col, ch) in line.iter().enumerate() {
            if *ch == '^' {
                caret_location = Some((row as i32, col as i32));
                break;
            }
        }
        if caret_location.is_some() {
            break;
        }
    }

    if let Some(mut caret) = caret_location {
        let directions = [(0, -1), (-1, 0), (0, 1), (1, 0)]; // left, up, right, down
        let mut direction_index = 1; 

        loop {
            let (row, col) = (caret.0 as usize, caret.1 as usize);
            if grid[row][col] == '#' {
                caret.0 -= directions[direction_index].0;
                caret.1 -= directions[direction_index].1;

                direction_index = (direction_index + 1) % 4;
            }

            grid[row][col] = 'X';

            caret.0 += directions[direction_index].0;
            caret.1 += directions[direction_index].1;

            if caret.0 < 0 || caret.0 >= grid.len() as i32 || caret.1 < 0 || caret.1 >= grid[0].len() as i32 {
                break;
            }

            if grid[caret.0 as usize][caret.1 as usize] == '#' {
                caret.0 -= directions[direction_index].0;
                caret.1 -= directions[direction_index].1;

                direction_index = (direction_index + 1) % 4;

                caret.0 += directions[direction_index].0;
                caret.1 += directions[direction_index].1;
            }

            grid[caret.0 as usize][caret.1 as usize] = '^';


        }
    } 

    let x_count = grid.iter().flatten().filter(|&&ch| ch == 'X').count();
    println!("x: {}", x_count);
}


fn simulate_guard(grid: &mut Vec<Vec<char>>, start_position: (i32, i32)) -> bool {
    let directions = [(0, -1), (-1, 0), (0, 1), (1, 0)]; 
    let mut direction_index = 1;
    let mut caret = start_position;
    let mut visited_positions = HashSet::new();

    loop {
        let (row, col) = (caret.0 as usize, caret.1 as usize);
        if grid[row][col] == '#' || grid[row][col] == 'O' {
            caret.0 -= directions[direction_index].0;
            caret.1 -= directions[direction_index].1;

            direction_index = (direction_index + 1) % 4;
        }

        grid[row][col] = 'X';

        caret.0 += directions[direction_index].0;
        caret.1 += directions[direction_index].1;

        if caret.0 < 0 || caret.0 >= grid.len() as i32 || caret.1 < 0 || caret.1 >= grid[0].len() as i32 {
            return false;
        }
        
        while grid[caret.0 as usize][caret.1 as usize] == '#' || grid[caret.0 as usize][caret.1 as usize] == 'O' {
            caret.0 -= directions[direction_index].0;
            caret.1 -= directions[direction_index].1;

            direction_index = (direction_index + 1) % 4;

            caret.0 += directions[direction_index].0;
            caret.1 += directions[direction_index].1;
        }


        let current_position = (caret.0, caret.1, direction_index);
        if visited_positions.contains(&current_position) {
            return true;
        } else {
            visited_positions.insert(current_position);
        }

        grid[caret.0 as usize][caret.1 as usize] = '^';
    }
}

pub fn run2(input: &[String]) {
    let mut grid: Vec<Vec<char>> = input.iter().map(|line| line.chars().collect()).collect();
    let mut caret_location = None;
    let mut original_path = HashSet::new();

    for (row, line) in grid.iter().enumerate() {
        for (col, ch) in line.iter().enumerate() {
            if *ch == '^' {
                caret_location = Some((row as i32, col as i32));
                break;
            }
        }
        if caret_location.is_some() {
            break;
        }
    }

    if let Some(start_position) = caret_location {
        let mut caret = start_position;
        let directions = [(0, -1), (-1, 0), (0, 1), (1, 0)];
        let mut direction_index = 1; 

        let mut grid_clone = grid.clone();

        loop {
            let (row, col) = (caret.0 as usize, caret.1 as usize);
            while grid_clone[caret.0 as usize][caret.1 as usize] == '#' || grid_clone[caret.0 as usize][caret.1 as usize] == 'O' {
                caret.0 -= directions[direction_index].0;
                caret.1 -= directions[direction_index].1;
    
                direction_index = (direction_index + 1) % 4;
    
                caret.0 += directions[direction_index].0;
                caret.1 += directions[direction_index].1;
            }
    

            original_path.insert((row as i32, col as i32));

            caret.0 += directions[direction_index].0;
            caret.1 += directions[direction_index].1;

            if caret.0 < 0 || caret.0 >= grid_clone.len() as i32 || caret.1 < 0 || caret.1 >= grid_clone[0].len() as i32 {
                break;
            }

            if grid_clone[caret.0 as usize][caret.1 as usize] == '#' {
                caret.0 -= directions[direction_index].0;
                caret.1 -= directions[direction_index].1;

                direction_index = (direction_index + 1) % 4;

                caret.0 += directions[direction_index].0;
                caret.1 += directions[direction_index].1;
            }

            grid_clone[caret.0 as usize][caret.1 as usize] = '^';
        }

        let mut possible_positions = Vec::new();

        for &(row, col) in &original_path {
            if grid[row as usize][col as usize] == '.' {
                grid[row as usize][col as usize] = 'O';

                if simulate_guard(&mut grid.clone(), start_position.clone()) {
                    possible_positions.push((row, col));
                }

                grid[row as usize][col as usize] = '.';
            }
        }

        let mut caret_up = start_position;
        caret_up.1 -= 1;
        if caret_up.1 >= 0 {
            grid[start_position.1 as usize][start_position.0 as usize] = 'O';
            if simulate_guard(&mut grid.clone(), caret_up) {
                possible_positions.push(start_position);
            }
            grid[start_position.1 as usize][start_position.0 as usize] = '.';
        }

        for &(row, col) in &possible_positions {
            grid[row as usize][col as usize] = 'O';
        }

        let obstruction_count = grid.iter().flatten().filter(|&&ch| ch == 'O').count();
        println!("obstructions: {}", obstruction_count);
    } 
}