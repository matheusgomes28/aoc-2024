pub fn read_input(input: &str) -> Vec<Vec<i32>> {
    input
        .lines()
        .map(|line| line.split_whitespace())
        .map(|vec| vec.map(|e| e.parse::<i32>().unwrap()))
        .map(|vec| vec.collect::<Vec<i32>>())
        .collect::<Vec<_>>()
}

// Part one: all steps must be safe
pub fn is_report_safe(report: &[i32]) -> bool {
    let diff = report
        .windows(2)
        .map(|list| (list[0] - list[1]) - 3)
        .collect::<Vec<_>>();

    diff
        .iter()
        .all(|a| (-2..=0).contains(a))
    || diff
        .iter()
        .all(|a| (-6..=-4).contains(a))
}

// Part two: tolerance of one unsafe step
pub fn is_report_mostly_safe(report: &[i32]) -> bool {
    let mut sublists = Vec::<Vec<i32>>::new();
    for i in 0..report.len() {
        let mut sublist = report.to_vec();
        sublist.remove(i);
        sublists.push(sublist);
    }

    sublists.iter().any(|r| is_report_safe(r))
}

fn main() {
    
    // Part one: No dampner applied, all steps must
    // be safe
    let reports = read_input(include_str!("input.in"));
    let safe_reports = reports
        .iter()
        .filter(|r| is_report_safe(r))
        .count();
    println!("day two part one: {}", safe_reports);

    // Part two: A single unsafe step is allowed
    let mostly_safe_reports = reports
        .iter()
        .filter(|r| is_report_mostly_safe(r));

    let test = mostly_safe_reports.cloned().collect::<Vec<_>>();
    println!("day two part two: {}", test.len());
}
