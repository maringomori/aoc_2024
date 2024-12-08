
use std::collections::HashMap;


fn draw_antinodes(grid: &mut Vec<Vec<char>>, a: (usize, usize), b: (usize, usize)) {
    let (x0, y0) = a;
    let (x1, y1) = b;

    let dx = x1 as isize - x0 as isize;
    let dy = y1 as isize - y0 as isize;

    let anti1 = ((x1 as isize + dx) as isize, (y1 as isize + dy) as isize);
    let anti2 = ((x0 as isize - dx) as isize, (y0 as isize - dy) as isize);

    if anti1.0 >= 0 && anti1.1 >= 0 && (anti1.0 as usize) < grid.len() && (anti1.1 as usize) < grid[0].len() {
        grid[anti1.0 as usize][anti1.1 as usize] = '#';
    }

    if anti2.0 >= 0 && anti2.1 >= 0 && (anti2.0 as usize) < grid.len() && (anti2.1 as usize) < grid[0].len() {
        grid[anti2.0 as usize][anti2.1 as usize] = '#';
    }
}

fn draw_antinodes2(grid: &mut Vec<Vec<char>>, a: (usize, usize), b: (usize, usize)) {
    let (x0, y0) = a;
    let (x1, y1) = b;

    let dx = x1 as isize - x0 as isize;
    let dy = y1 as isize - y0 as isize;

    let mut x = x1 as isize;
    let mut y = y1 as isize;
    while x >= 0 && y >= 0 && (x as usize) < grid.len() && (y as usize) < grid[0].len() {
        grid[x as usize][y as usize] = '#';
        x += dx;
        y += dy;
    }

    x = x1 as isize;
    y = y1 as isize;
    while x >= 0 && y >= 0 && (x as usize) < grid.len() && (y as usize) < grid[0].len() {
        grid[x as usize][y as usize] = '#';
        x -= dx;
        y -= dy;
    } 
}


pub fn run1(input: &[String]) {
    let mut positions: HashMap<char, Vec<(usize, usize)>> = HashMap::new();
    let mut grid: Vec<Vec<char>> = input.iter().map(|line| line.chars().collect()).collect();

    for (row_index, line) in input.iter().enumerate() {
        for (col_index, ch) in line.chars().enumerate() {
            if ch != '.' {
                positions.entry(ch).or_insert_with(Vec::new).push((row_index, col_index));
            }
        }
    }

    for (ch, pos_list) in &positions {
        for i in 0..pos_list.len() {
            for j in i + 1..pos_list.len() {
                draw_antinodes(&mut grid, pos_list[i], pos_list[j]);
            }
        }
    }

    let count = grid.iter().flatten().filter(|&&c| c == '#').count();
    println!("Count of #: {}", count);

}



pub fn run2(input: &[String]) {
    let mut positions: HashMap<char, Vec<(usize, usize)>> = HashMap::new();
    let mut grid: Vec<Vec<char>> = input.iter().map(|line| line.chars().collect()).collect();

    for (row_index, line) in input.iter().enumerate() {
        for (col_index, ch) in line.chars().enumerate() {
            if ch != '.' {
                positions.entry(ch).or_insert_with(Vec::new).push((row_index, col_index));
            }
        }
    }

    for (ch, pos_list) in &positions {
        for i in 0..pos_list.len() {
            for j in i + 1..pos_list.len() {
                draw_antinodes2(&mut grid, pos_list[i], pos_list[j]);
            }
        }
    }

    let count = grid.iter().flatten().filter(|&&c| c == '#').count();
    println!("Count of #: {}", count);
}
