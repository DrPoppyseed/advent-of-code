use std::fs;

pub fn run() {
    const PATH: &str = "inputs/2.txt";
    let content = fs::read_to_string(PATH).unwrap();
    let reports = content
        .lines()
        .map(|l| {
            l.trim()
                .split(' ')
                .flat_map(|num| num.parse::<i32>().ok())
                .collect::<Vec<i32>>()
        })
        .collect::<Vec<_>>();

    let ans = reports
        .into_iter()
        .fold(0, |acc, report| acc + calc_next(report));

    println!("ans: {ans}");
}

fn calc_next(report: Vec<i32>) -> i32 {
    // since we'd already know the length of the vector we're receiving, we
    // can probably implement all this only using slices, but I don't want to
    // bother with lifetime issues down the line so I'm cloning everything and
    // using vectors everywhere.
    let mut diffs: Vec<Vec<i32>> = vec![report];

    while diffs[diffs.len() - 1].iter().any(|v| v != &0) {
        let parent_diffs = diffs[diffs.len() - 1].clone();
        let mut diff: Vec<i32> = vec![];
        let mut prev = parent_diffs[0];
        for cur in parent_diffs[1..].into_iter() {
            diff.push(cur - prev);
            prev = *cur;
        }
        diffs.push(diff);
    }

    // println!("diffs: {diffs:?}");

    let next = diffs
        .into_iter()
        .fold(0, |acc, cur| acc + cur.last().unwrap());

    next
}
