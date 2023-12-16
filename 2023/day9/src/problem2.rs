use std::fs;

pub fn run() {
    let ans = fs::read_to_string("inputs/2.txt")
        .unwrap()
        .lines()
        .map(|l| {
            l.split(' ')
                .flat_map(|num| num.parse::<i32>().ok())
                .collect::<Vec<i32>>()
        })
        .fold(0, |acc, report| acc + calc_next(report));

    println!("ans: {ans}");
}

fn calc_next(report: Vec<i32>) -> i32 {
    let mut diffs: Vec<Vec<i32>> = vec![report];

    while diffs[diffs.len() - 1].iter().any(|v| v != &0) {
        let parent_diffs = diffs[diffs.len() - 1].clone();
        let mut diff: Vec<i32> = vec![];
        let mut prev = parent_diffs[0];

        parent_diffs[1..].iter().for_each(|cur| {
            diff.push(cur - prev);
            prev = *cur;
        });

        diffs.push(diff);
    }

    let diffs = diffs.into_iter().rev().collect::<Vec<_>>();

    let mut prev: i32 = 0;
    for (idx, _) in diffs.clone()[..diffs.len() - 1].into_iter().enumerate() {
        prev = diffs[idx + 1].first().unwrap() - prev;
    }

    prev
}
