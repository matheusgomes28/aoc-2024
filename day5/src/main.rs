use std::collections::{HashMap, HashSet};

fn construct_order_tree(input: &str) -> HashMap<i32, HashSet<i32>> {

    let mut tree = HashMap::<i32, HashSet<i32>>::new();
    for (before, after) in input
        .lines()
        .map(|l| l.split('|').collect::<Vec<_>>())
        .map(|split| (split[0].parse::<i32>().unwrap(), split[1].parse::<i32>().unwrap())) {

        match tree.get_mut(&before) {
            Some(depedencies) => {
                depedencies.insert(after);
            },
            _ => {
                tree.insert(before, HashSet::<_>::from([after]));
            }
        }
    }

    tree
}

fn construct_updates(input: &str) -> Vec<Vec<i32>> {
    input
        .lines()
        .map(|l| l.split(',').collect::<Vec<_>>())
        .map(|d| d.into_iter().map(|v| v.parse::<i32>().unwrap()))
        .map(|d| d.collect::<Vec<_>>())
        .collect::<Vec<_>>()
}

fn check_update(updates: &[i32], dependencies: &HashMap<i32, HashSet<i32>>) -> bool {

    for i in 0..updates.len() {

        let current = updates[i];

        // Check current comes before all the following items in its list
        // is this necessary?
        // Check current items comes after each prev item in their lists
        for j in i..updates.len() {
            let after = updates[j];
            let deps = dependencies
                .get(&after)
                .cloned()
                .unwrap_or_default();
            
            if deps.contains(&current) {
                return false;
            }
        }
    }

    true
}

// Same as check_update but it swaps the wrong values,
// and has a while outer loop to rinse and repeat until
// it's all looking good
fn correct_update(updates: &[i32], dependencies: &HashMap<i32, HashSet<i32>>) -> Vec<i32> {
    let mut new_updates = updates.to_vec();

    let mut corrected = true;
    'outer: while corrected {
        corrected = false;
        for i in 0..new_updates.len() {
    
            let current = new_updates[i];
            for j in i..new_updates.len() {
                let after = new_updates[j];
                let deps = dependencies
                    .get(&after)
                    .cloned()
                    .unwrap_or_default();
                
                if deps.contains(&current) {
                    new_updates.swap(i, j);
                    corrected = true;
                    continue 'outer;
                }
            }
        }
    }

    assert!(check_update(&new_updates, dependencies));
    new_updates
}

fn get_middle(input: &[i32]) -> i32 {
    assert!((input.len() % 2) == 1);
    input[input.len()/2]
}

fn main() {

    let tree_input = include_str!("day5_tree.in");
    let tree = construct_order_tree(tree_input);
    let updates_input = include_str!("day5_update.in");
    let updates = construct_updates(updates_input);

    let valid_updates = updates
        .iter()
        .filter(|u| check_update(u, &tree))
        .map(|u| u.clone())
        .collect::<Vec<_>>();

    let invalid_updates = updates
        .iter()
        .filter(|u| !check_update(u, &tree))
        .map(|u| u.clone())
        .map(|u| correct_update(&u, &tree))
        .collect::<Vec<_>>();

    let sum_mids: i32 = valid_updates
        .iter()
        .map(|v| get_middle(v))
        .sum();

    let invalid_sum_mids: i32 = invalid_updates
        .iter()
        .map(|v| get_middle(v))
        .sum();


    println!("Part one valid updates: {}", valid_updates.len());
    println!("Part one sum of mids: {}", sum_mids);

    println!("Part two invalid updates: {}", invalid_updates.len());
    println!("Part two sum of mids: {}", invalid_sum_mids);

}
