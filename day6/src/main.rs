use std::collections::HashSet;

use anyhow::bail;
use itertools::Itertools;
use ndarray::Array2;

#[derive(Clone, Debug)]
enum Cell {
    Wall, // a wall
    Unvisited, // unvisited cell
    Visited(HashSet<(i32, i32)>), // visited cell
}

#[derive(Clone, Debug)]
enum RunType {
    Normal,
    Loop
}

fn rotate_guard(dir: (i32, i32)) -> (i32, i32) {
    (dir.1, -dir.0)
}

#[derive(Clone, Debug)]
struct Guard {
    /// Where it is currently
    pos: (i32, i32),
    /// Where it is walking towards
    dir: (i32, i32),
}

fn char_to_guard(c: char, index: (usize, usize)) -> anyhow::Result<Guard> {
    let pos = (index.0 as i32, index.1 as i32);
    match c {
        '^' => Ok(Guard{dir: (-1, 0), pos}),
        '>' => Ok(Guard{dir: (0, 1), pos}),
        'v' => Ok(Guard{dir: (1, 0), pos}),
        '<' => Ok(Guard{dir: (0, -1), pos}),
        _ => {
            bail!("invalid input character")
        }
    }
}

fn char_to_cell(c: char) -> anyhow::Result<Cell> {
    match c {
        '#' => Ok(Cell::Wall),
        '.' => Ok(Cell::Unvisited),
        _ => bail!("invalid input character")
    }
}

fn get_grid(input: &str) -> anyhow::Result<(Guard, Array2<Cell>)> {

    // Split on new lines to get the vector
    let lines = input
        .lines()
        .map(|l| l.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    // Make sure all rows have the same size
    let col_size = lines.len();
    let row_size = lines[0].clone().len();

    if !lines.iter().map(|l| l.len()).all(|s| s == row_size) {
        bail!("not all input rows have the same size")
    }

    // May throw if `char_to_cell(...)` does not convert
    let lines_flattened = lines.into_iter().flatten().collect::<Vec<_>>();
    let mut lines_array = Array2::<char>::from_shape_vec((col_size, row_size),lines_flattened)?;

    // Find the guard
    let (guard_pos, &guard_char) = lines_array
        .indexed_iter()
        .find(|(_, value)| ['^', '>', '<', 'v'].contains(value))
        .unwrap();
    let guard = char_to_guard(guard_char, guard_pos)?;

    // convert the array of char to array of Cell
    lines_array[guard_pos] = '.';
    let grid = lines_array
        .map(|c| char_to_cell(*c).unwrap());

    Ok((guard, grid))
}

fn guard_walk(grid: &mut Array2<Cell>, guard: &mut Guard) -> anyhow::Result<RunType> {
    
    // The following should be in a loop
    let shape = grid.shape();
    let (rows, cols) = (shape[0], shape[1]);
    
    // TODO : Maybe it's worth holding the next position as
    // TODO : a tuple to reuse it around
    while [
        guard.pos.0 + guard.dir.0 < rows as i32,
        guard.pos.1 + guard.dir.1 < cols as i32,
        guard.pos.0 + guard.dir.0 >= 0,
        guard.pos.1 + guard.dir.1 >= 0,
    ].iter().all(|a| *a) {

        let mut next_cell = &mut grid[((guard.pos.0 + guard.dir.0) as usize, (guard.pos.1 + guard.dir.1) as usize)];
        match &mut next_cell {
            Cell::Wall => {
                // need to update guard direction without rotating
                guard.dir = rotate_guard(guard.dir);
                continue;
            },
            Cell::Visited(ref mut dirs) => {
                // we have detected a loop only if the current
                // direction is already in the set
                if dirs.contains(&guard.dir)  {
                    return Ok(RunType::Loop);
                } else{
                    dirs.insert(guard.dir);
                }
            },
            _ => {}
        }

        let current_cell= grid
            .get_mut((guard.pos.0 as usize, guard.pos.1 as usize))
            .unwrap();
        if let Cell::Visited(dirs) = current_cell {
            dirs.insert(guard.dir);
        } else {
            grid[(guard.pos.0 as usize, guard.pos.1 as usize)] = Cell::Visited(HashSet::<_>::from([guard.dir]));
        }

        guard.pos = (guard.pos.0 + guard.dir.0, guard.pos.1 + guard.dir.1);
    }

    grid[(guard.pos.0 as usize, guard.pos.1 as usize)] = Cell::Visited(HashSet::<_>::from([guard.dir]));
    Ok(RunType::Normal)
}


fn try_main() -> anyhow::Result<()> {
    let input_str = include_str!("day6.in");
    let (original_guard, original_grid) = get_grid(input_str)?;

    let mut grid = original_grid.clone();
    let mut guard = original_guard.clone();
    guard_walk(&mut grid, &mut guard)?;

    let visited_cells = grid
        .iter()
        .filter(|c| matches!(c, Cell::Visited(_)))
        .count();

    println!("Part one visited: {}", visited_cells);

    // Part two: brute forca all possible positions
    // initial pos: 95,68
    let (guard_row, guard_col) = (95, 68);
    let shape = grid.shape();
    let (rows, cols) = (shape[0], shape[1]);

    let mut count = 0;
    for (r, c) in (0..rows).cartesian_product(0..cols)
        .filter(|&(r, c)|  (r, c) != (guard_row, guard_col)) {

        let mut new_grid = original_grid.clone();
        new_grid[(r, c)] = Cell::Wall;
        let mut new_guard = original_guard.clone();

        if let RunType::Loop = guard_walk(&mut new_grid, &mut new_guard)? {
            count += 1;
        }
    }

    println!("Part two: {}", count);
    Ok(())
}

fn main() {
    match try_main() {
        Ok(()) => {
            println!("successfully finished the program");
        }
        Err(e) => {
            eprintln!("unsuccessfully finished the program: {}", e);
        }
    }
}
