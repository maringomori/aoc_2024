use std::collections::HashSet;

fn find_paths(grid: &[Vec<u32>], i: usize, j: usize, path: &mut Vec<(usize, usize)>, visited: &mut Vec<Vec<bool>>, unique_paths: &mut HashSet<(usize, usize)>) {
    if i >= grid.len() || j >= grid[0].len() || visited[i][j] {
        return;
    }

    path.push((i, j));
    visited[i][j] = true;

    if grid[i][j] == 9 {
        let end_position = *path.last().unwrap();
        unique_paths.insert(end_position);
        let start_position = path.first().unwrap();
        let steps = path.len() - 1;
    } else {
        let current_value = grid[i][j];
        let directions = [(0, 1), (1, 0), (0, -1), (-1, 0)];

        for &(di, dj) in &directions {
            let new_i = i.wrapping_add(di as usize);
            let new_j = j.wrapping_add(dj as usize);

            if new_i < grid.len() && new_j < grid[0].len() && grid[new_i][new_j] == current_value + 1 {
                find_paths(grid, new_i, new_j, path, visited, unique_paths);
            }
        }
    }

    path.pop();
    visited[i][j] = false;
}

fn find_paths2(grid: &[Vec<u32>], i: usize, j: usize, path: &mut Vec<(usize, usize)>, visited: &mut Vec<Vec<bool>>, unique_paths: &mut HashSet<Vec<(usize, usize)>>) {
    if i >= grid.len() || j >= grid[0].len() || visited[i][j] {
        return;
    }

    path.push((i, j));
    visited[i][j] = true;

    if grid[i][j] == 9 {
        unique_paths.insert(path.clone());
        let start_position = path.first().unwrap();
        let end_position = path.last().unwrap();
        let steps = path.len() - 1;
        println!("Path: {:?}, Steps: {}, Start: {:?}, End: {:?}", path, steps, start_position, end_position);
    } else {
        let current_value = grid[i][j];
        let directions = [(0, 1), (1, 0), (0, -1), (-1, 0)];

        for &(di, dj) in &directions {
            let new_i = i.wrapping_add(di as usize);
            let new_j = j.wrapping_add(dj as usize);

            if new_i < grid.len() && new_j < grid[0].len() && grid[new_i][new_j] == current_value + 1 {
                find_paths2(grid, new_i, new_j, path, visited, unique_paths);
            }
        }
    }

    path.pop();
    visited[i][j] = false;
}

pub fn run1(input: &[String]) {
    let grid: Vec<Vec<u32>> = input
        .iter()
        .map(|line| line.chars().map(|c| c.to_digit(10).unwrap()).collect())
        .collect();

    let mut positions = Vec::new();

    for (i, row) in grid.iter().enumerate() {
        for (j, &value) in row.iter().enumerate() {
            if value == 0 {
                positions.push((i, j));
            }
        }
    }

    let mut sum = 0;
    for &(start_i, start_j) in &positions {
        let mut path = Vec::new();
        let mut visited = vec![vec![false; grid[0].len()]; grid.len()];
        let mut unique_paths = HashSet::new();
        find_paths(&grid, start_i, start_j, &mut path, &mut visited, &mut unique_paths);
        sum += unique_paths.len();
    }

    println!("Sum: {}", sum);
}


pub fn run2(input: &[String]) {
    let grid: Vec<Vec<u32>> = input
        .iter()
        .map(|line| line.chars().map(|c| c.to_digit(10).unwrap()).collect())
        .collect();

    let mut positions = Vec::new();

    for (i, row) in grid.iter().enumerate() {
        for (j, &value) in row.iter().enumerate() {
            if value == 0 {
                positions.push((i, j));
            }
        }
    }

    let mut sum = 0;
    for &(start_i, start_j) in &positions {
        let mut path = Vec::new();
        let mut visited = vec![vec![false; grid[0].len()]; grid.len()];
        let mut unique_paths = HashSet::new();
        find_paths2(&grid, start_i, start_j, &mut path, &mut visited, &mut unique_paths);
        sum += unique_paths.len();
    }

    println!("Sum: {}", sum);
}
