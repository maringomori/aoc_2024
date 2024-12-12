use std::collections::HashSet;

fn dfs(grid: &Vec<Vec<char>>, i: usize, j: usize, target: char, visited: &mut HashSet<(usize, usize)>, group: &mut Vec<(usize, usize)>) -> usize {
    if i >= grid.len() || j >= grid[i].len() || grid[i][j] != target || visited.contains(&(i, j)) {
        return 0;
    }

    visited.insert((i, j));
    group.push((i, j));

    let directions = vec![(0, 1), (1, 0), (0, -1), (-1, 0)];
    let mut sides = 0;

    for (di, dj) in directions {
        let ni = i as isize + di;
        let nj = j as isize + dj;
        if ni >= 0 && nj >= 0 {
            if ni as usize >= grid.len() || nj as usize >= grid[ni as usize].len() || grid[ni as usize][nj as usize] != target {
                sides += 1;
            } else {
                sides += dfs(grid, ni as usize, nj as usize, target, visited, group);
            }
        } else {
            sides += 1;
        }
    }

    sides
}

fn calculate_matching_sides(grid: &Vec<Vec<char>>, groups: &Vec<Vec<(usize, usize)>>) -> Vec<usize> {
    let mut matching_sides = Vec::new();

    for group in groups {
        let region: HashSet<(i32, i32)> = group.iter().map(|&(i, j)| (i as i32, j as i32)).collect();
        let sides = count_region_sides(&region);
        matching_sides.push(sides);
    }

    matching_sides
}

fn count_region_sides(region: &HashSet<(i32, i32)>) -> usize {
    let mut side_count = 0;
    let directions = vec![(0, 1), (1, 0), (0, -1), (-1, 0)];
    for dir in directions {
        let mut sides = HashSet::new();
        for pos in region {
            let tmp = (pos.0 + dir.0, pos.1 + dir.1);
            if !region.contains(&tmp) {
                sides.insert(tmp);
            }
        }
        let mut remove = HashSet::new();
        for side in &sides {
            let mut tmp = (side.0 + dir.1, side.1 + dir.0);
            while sides.contains(&tmp) {
                remove.insert(tmp);
                tmp = (tmp.0 + dir.1, tmp.1 + dir.0);
            }
        }
        side_count += sides.len() - remove.len();
    }

    side_count
}

pub fn run1(input: &[String]) {
    let grid: Vec<Vec<char>> = input.iter()
        .map(|line| line.chars().collect())
        .collect();

    let mut visited = HashSet::new();
    let mut groups = Vec::new();
    let mut sides_count = Vec::new();

    for i in 0..grid.len() {
        for j in 0..grid[i].len() {
            if !visited.contains(&(i, j)) {
                let mut group = Vec::new();
                let sides = dfs(&grid, i, j, grid[i][j], &mut visited, &mut group);
                if !group.is_empty() {
                    groups.push(group);
                    sides_count.push(sides);
                }
            }
        }
    }
    let mut total_price = 0;
    for (group, sides) in groups.iter().zip(sides_count.iter()) {
        total_price += group.len() * sides;
    }

    println!("{}", total_price);
}


pub fn run2(input: &[String]) {
    let grid: Vec<Vec<char>> = input.iter()
        .map(|line| line.chars().collect())
        .collect();

    let mut visited = HashSet::new();
    let mut groups = Vec::new();

    for i in 0..grid.len() {
        for j in 0..grid[i].len() {
            if !visited.contains(&(i, j)) {
                let mut group = Vec::new();
                dfs(&grid, i, j, grid[i][j], &mut visited, &mut group);
                if !group.is_empty() {
                    groups.push(group);
                }
            }
        }
    }

    let matching_sides = calculate_matching_sides(&grid, &groups);


    let mut total_price = 0;
    for (group, sides) in groups.iter().zip(matching_sides.iter()) {
        total_price += group.len() * sides;
    }
    println!("{}", total_price);
}

