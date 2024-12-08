use anyhow::bail;
use ndarray::Array2;

const DIRECTIONS : &[(i32, i32)] = &[
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

fn get_deltas(direction: &(i32, i32), steps: usize) -> Vec<(i32, i32)> {

    let mut deltas = vec![(0, 0)];
    for i in 1..steps {
        let (row, col) = direction.clone();
        deltas.push((row * i as i32, col * i as i32));
    }

    deltas
}

fn find_str(input: &Array2<char>, pattern: &str) -> usize {

    // create the right offsets for each direction
    let directions = DIRECTIONS
        .iter()
        .map(|d| get_deltas(d, pattern.len()))
        .collect::<Vec<_>>();

    let shape = input.shape();
    assert!(shape.len() == 2);
    let (rows, cols) = (shape[0], shape[1]);


    let allowed_patterns = &[
        pattern.chars().collect::<Vec<_>>(),
        // pattern.chars().rev().collect::<Vec<_>>(),
    ];
    
    let mut count = 0;
    for (row, col) in  (0..rows).flat_map(|x| (0..cols).map(move |y| (x, y))) {
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

        count += strings.iter().filter(|str| allowed_patterns.contains(str)).count();
    }

    count
}

fn try_main() -> anyhow::Result<()> {

    // Part one: find all 'XMAS' str in the input
    let input = include_str!("day4.in");
    let text_array =  into_matrix(input)?;
    let count = find_str(&text_array, "XMAS");
    println!("Day4 part one: {:?}", count);

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
