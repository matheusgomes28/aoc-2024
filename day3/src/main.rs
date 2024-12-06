use regex::Regex;

fn get_instruction_str(input: &str) -> anyhow::Result<String> {

    let mut new_input = input.to_string();
    new_input.insert_str(0, "do()");
    new_input.push_str("don't()");

    // This will give me strings with don't() in it
    // we need to ignore everything after that
    Ok(new_input
        .split("do()")
        .map(|substr| substr[..substr.find("don't").unwrap_or(substr.len())].to_string())
        .fold(String::new(), |a, b| format!("{}{}", a, b)))
}

fn get_multiply_pairs(input: &str) -> anyhow::Result<Vec<(i32, i32)>> {
    let pattern = r"mul\((?<a>[0-9]{1,3}),(?<b>[0-9]{1,3})\)";
    let re = Regex::new(pattern)?;

    let captures = re.captures_iter(input);

    // Vec<(str, str)>
    Ok(captures
        .map(|c| (c["a"].parse::<i32>().unwrap(), c["b"].parse::<i32>().unwrap()))
        .collect::<Vec<_>>())
}

fn try_main() -> anyhow::Result<()> {

    // Part one: find all valid mul(X,Y) instructions,
    // and operate and sum them up
    // Read the input
    // let input = include_str!("day3.in");
    let input = include_str!("day3.in");
    let valid_instructions = get_multiply_pairs(input)?;
    let sum_of_mul: i32 = valid_instructions
        .iter()
        .map(|(a, b)| a * b)
        .sum();
    println!("day3 part one: {}", sum_of_mul);


    // Part two: We only care for what's inside "do()" and "don't()"
    // strings lazily
    let do_instructions = get_instruction_str(&input)?;
    let valid_instructions = get_multiply_pairs(&do_instructions)?;
    let sum_of_mul: i32 = valid_instructions
        .iter()
        .map(|(a, b)| a * b)
        .sum();
    println!("day3 part two: {}", sum_of_mul);

    Ok(())
}

fn main() {
    match try_main() {
        Ok(_) => {
            println!("Finished successfully");
        },
        Err(e) => {
            eprintln!("Finished with error: {}", e);
        }
    }
}
