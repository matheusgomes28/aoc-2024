use std::collections::HashMap;

pub fn read_input(input_text: &str) -> (Vec<i32>, Vec<i32>) {

    let mut list_one: Vec<i32> = vec![];
    let mut list_two: Vec<i32> = vec![];

    for line in input_text.lines()  {
        // split on spaces
        let split_text = line.split(' ');
        assert!(split_text.clone().count() > 2);

        let first_elem = split_text.clone().next().unwrap();
        let second_elem = split_text.last().unwrap();

        list_one.push(first_elem.parse::<i32>().unwrap());
        list_two.push(second_elem.parse::<i32>().unwrap());
    }

    (list_one, list_two)
}

pub fn calculate_difference(list_one: &mut Vec<i32>, list_two: &mut Vec<i32>) -> u32 {
    list_one.sort();
    list_two.sort();

    list_one
        .iter()
        .zip(list_two)
        .map(|(&a, &mut b)| (a - b).abs() as u32)
        .sum()
}

pub fn elem_freq(list: &[i32]) -> HashMap<i32, u32> {

    let mut frequencies =  HashMap::<i32, u32>::new();
    for elem in list {
        match frequencies.get_mut(elem) {
            Some(v) => {
                *v = *v + 1;
            },
            None => {
                frequencies.insert(*elem, 1);
            }
        }
    }

    return frequencies
}


pub fn calculate_similarity(list_one: &[i32], list_two: &[i32]) -> u32 {

    let freq_l2 = elem_freq(list_two);

    let mut similarity: u32 = 0;
    for elem in list_one {
        
        let multiplier = freq_l2
            .get(elem)
            .cloned()
            .unwrap_or(0);

        similarity = similarity + multiplier * (*elem as u32);
    }

    similarity
}


fn main() {
    // Part one : get the list differences sum
    let (mut list_one, mut list_two) = read_input(include_str!("input.in"));
    let part_one_res= calculate_difference(&mut list_one, &mut list_two);
    println!("part one: {}", part_one_res);

    // Part two: get the similarity
    let part_two_res = calculate_similarity(&list_one, &list_two);
    println!("part two: {}", part_two_res);
}
