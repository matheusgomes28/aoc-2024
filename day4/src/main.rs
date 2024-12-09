use anyhow::bail;
use itertools::Itertools;
use ndarray::Array2;

const DIRECTIONS : &[(i32, i32)] = &[
    // Straights
    (0, -1),
    (0, 1),
    (1, 0),
    (-1, 0),

    // Diagonals
    (1, 1),
    (-1, 1),
    (1, -1),
    (-1, -1),
];

fn into_matrix(input: &str) -> anyhow::Result<Array2<char>> {

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

    let lines_flattened = lines.into_iter().flatten().collect::<Vec<_>>();
    let lines_array = Array2::<char>::from_shape_vec((col_size, row_size),lines_flattened)?;

    Ok(lines_array)
}

/// Finds all the possible positions by walking in the
/// given `direction`` from (0,0) (inclusively) when
/// taking `steps` steps.
/// 
/// ```
/// let direction = (1, 0);
/// let walk_dirs = get_deltas(direction, 4);
/// assert!(walk_dirs == vec![(0,0), (1,0), (2,0), (3,0)]);
/// ```
fn direction_walk(direction: &(i32, i32), steps: usize) -> Vec<(i32, i32)> {

    let mut deltas = vec![(0, 0)];
    for i in 1..steps {
        let (row, col) = *direction;
        deltas.push((row * i as i32, col * i as i32));
    }

    deltas
}

/// Counts the number of occurences of the given `pattern`
/// in the `input` array string.
fn count_str(input: &Array2<char>, pattern: &str) -> usize {

    // create the right offsets for each direction
    let directions = DIRECTIONS
        .iter()
        .map(|d| direction_walk(d, pattern.len()))
        .collect::<Vec<_>>();

    let shape = input.shape();
    assert!(shape.len() == 2);
    let (rows, cols) = (shape[0], shape[1]);


    let mut count = 0;
    for (row, col) in (0..rows).cartesian_product(0..cols) {
        // TODO : Probably need to explain what this does hehe
        // Keep the directions we can go
        let valid_positions = directions
            .iter()
            .map(|ds| ds.iter().map(|(r, c)| (row as i32 + r, col as i32 + c)))
            .filter(|ps| ps.clone().all(|(r, c)| (r < rows as i32) && (c < cols as i32) && (r >= 0) && (c >= 0)))
            .map(|ds| ds.collect::<Vec<_>>())
            .collect::<Vec<_>>();

        let strings = valid_positions
            .iter()
            .map(|ds| ds.iter().map(|(r, c)|  input[(*r as usize, *c as usize)]))
            .map(|chars| chars.collect::<Vec<_>>())
            .collect::<Vec<_>>();

        count += strings.iter().filter(|str| pattern.chars().collect::<Vec<_>>() == **str).count();
    }

    count
}

// Cannot be bothered making this generic
fn count_mas_cross(input: &Array2<char>) -> usize {

    let positions = &[
        (-1, -1),
        (0, 0),
        (1, 1),

        (1, -1),
        (0, 0),
        (-1, 1),
    ];

    let shape = input.shape();
    assert!(shape.len() == 2);
    let (rows, cols) = (shape[0], shape[1]);

    let mut count: usize = 0;
    for (row, col) in (0..rows).cartesian_product(0..cols) {
        // We just need to see if the window will be valid here
        let valid_positions = positions
            .iter()
            .map(|(r, c)| (row as i32 + r, col as i32 + c))
            .collect::<Vec<_>>();

        if !valid_positions
            .clone()
            .into_iter()
            .all(|(r, c)| (r < rows as i32) && (c < cols as i32) && (r >= 0) && (c >= 0)) {
            continue;
        }

        let roi = valid_positions
            .iter()
            .map(|&(r, c)| input[(r as usize, c as usize)])
            .collect::<Vec<_>>();

        // should be equal to MAS or SAM
        let allowed_patterns = vec![
            "MAS".chars().collect::<Vec<_>>(),
            "SAM".chars().collect::<Vec<_>>(),
        ];
        let diag1 = vec![roi[0], roi[1], roi[2]];
        let diag2 = vec![roi[3], roi[4], roi[5]];
        if allowed_patterns.contains(&diag1) && allowed_patterns.contains(&diag2) {
            count += 1;
        }
    }

    count
}

fn try_main() -> anyhow::Result<()> {

    // Part one: find all 'XMAS' str in the input
    let input = include_str!("day4.in");
    let text_array =  into_matrix(input)?;
    let count = count_str(&text_array, "XMAS");
    println!("Day4 part one: {:?}", count);


    // Part two: find all `MAS` in the shape of an X
    // in the data
    let count = count_mas_cross(&text_array);
    println!("Day4 part two: {:?}", count);
    Ok(())
}

fn main() {
    
    match try_main() {
        Ok(_) => {
            println!("successfully finished");
        },
        Err(e) => {
            eprintln!("finished with error {}", e);
        }
    }
}
