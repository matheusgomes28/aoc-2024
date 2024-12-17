use anyhow::{bail, Context};
use itertools::Itertools;

#[derive(Debug, Clone)]
struct Test {
    // The desired total
    total: i64,

    // All operands in a line
    operands: Vec<i64>,
}

#[derive(Debug, Clone)]
enum Operators {
    Plus,
    Mult,
    Concat,
}

fn get_operator_cartesian(dims: usize) -> Vec<Vec<Operators>> {
    assert!(dims > 0);

    (0..dims)
        .map(|_| [Operators::Plus, Operators::Mult, Operators::Concat])
        .multi_cartesian_product()
        .collect::<Vec<_>>()
}

fn apply_formula(operands: &Vec<i64>, operations: &Vec<Operators>) -> i64 {
    assert!(operands.len() > 0);
    assert!(
        operands.len() == operations.len() + 1,
        "mismatch in operand size"
    );

    // Get the first operand here and
    let mut res = operands[0];
    for (next_operand, operator) in operands.iter().skip(1).zip(operations) {
        match operator {
            Operators::Plus => {
                res += next_operand;
            }
            Operators::Mult => {
                res *= next_operand;
            }
            Operators::Concat => {
                // This can be simplified with log arithmetic
                // plus basic digit shift arithmetic, much faster
                // than converting to strings for sure
                let res_str = res.to_string();
                let operand_str = next_operand.to_string();
                res = format!("{}{}", res_str, operand_str)
                    .parse::<i64>()
                    .unwrap();
            }
        }
    }

    res
}

fn find_solutions(test: &Test) -> anyhow::Result<Vec<Vec<Operators>>> {
    if test.operands.len() == 0 {
        bail!("invalid number of operands")
    }

    let num_gaps = test.operands.len() - 1;
    let operations = get_operator_cartesian(num_gaps);

    let valid_solutions = operations
        .into_iter()
        .filter(|o| apply_formula(&test.operands, o) == test.total)
        .collect::<Vec<_>>();

    if valid_solutions.is_empty() {
        bail!("test doesn't have solution")
    }

    Ok(valid_solutions)
}

fn parse_test_line(test_line: &str) -> anyhow::Result<Test> {
    // Vec after we split on ':'
    let mut split = test_line.split(':');

    // Split on the ":" to get the total
    println!("processing line: {}", test_line);
    let total = split
        .next()
        .context("could not find the result in str")?
        .parse::<i64>()?;

    let operands = split
        .next()
        .context("could not find operands in str")?
        .trim()
        .split(' ')
        .map(|num_str| num_str.parse::<i64>().unwrap())
        .collect::<Vec<_>>();

    // With remaining line, strip and split on " " to get
    // each possible operand
    Ok(Test { total, operands })
}

fn parse_input(input: &str) -> anyhow::Result<Vec<Test>> {
    let lines = input
        .lines()
        .map(|l| parse_test_line(l))
        .collect::<Vec<_>>();

    if !lines.iter().all(|test| test.is_ok()) {
        bail!("could not convert all lines to a test")
    }

    Ok(lines
        .into_iter()
        .map(|test| test.unwrap())
        .collect::<Vec<_>>())
}

fn try_main() -> anyhow::Result<()> {
    // Parse the input
    let tests = parse_input(include_str!("day7.in"))?;

    let valid_tests = tests
        .iter()
        .filter(|t| find_solutions(&t).is_ok())
        .collect::<Vec<_>>();

    println!(
        "Part one sum: {}",
        valid_tests.iter().map(|t| t.total).sum::<i64>()
    );
    Ok(())
}

fn main() {
    match try_main() {
        Ok(_) => {
            println!("program finished successfully!");
        }
        Err(e) => {
            eprintln!("program crashed: {}", e);
        }
    }
}
