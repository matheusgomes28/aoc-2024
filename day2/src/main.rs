pub fn read_input(input: &str) -> Vec<Vec<i32>> {
    input
        .lines()
        .map(|line| line.split_whitespace())
        .map(|vec| vec.map(|e| e.parse::<i32>().unwrap()))
        .map(|vec| vec.collect::<Vec<i32>>())
        .collect::<Vec<_>>()
}

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

fn main() {
    let reports = read_input(include_str!("input.in"));
    let safe_reports = reports
        .iter()
        .filter(|r| is_report_safe(r))
        .count();

    println!("day two part one: {}", safe_reports);
}
