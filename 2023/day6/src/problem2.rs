// dealing with huge numbers, basically, we only care about the left and right edges.
// going through each number from head and tail and stopping once it finds a
// valid number. we just subtract the tail from the head to get the number of
// valid combinations. This should eliminate, like, 90% of the work right?
use std::fs;

pub fn run() {
    let path = "inputs/2.txt";

    let mut arrs: Vec<usize> = vec![];
    let mut lines = fs::read_to_string(path)
        .unwrap()
        .lines()
        .map(String::from)
        .collect::<Vec<_>>()
        .into_iter();

    while let Some(line) = lines.next() {
        arrs.push(
            line.split(' ')
                .filter_map(|c| c.parse::<usize>().ok())
                .map(|d| d.to_string())
                .collect::<String>()
                .parse::<usize>()
                .unwrap(),
        );
    }

    let (time, threshold) = (arrs[0], arrs[1]);
    println!("time: {time}, threshold: {threshold}");
    let mut head = 0;
    let mut tail = 0;

    // start from head
    for i in 0..time {
        if calc(threshold, time, i) {
            head = i;
            break;
        }
    }
    println!("head: {head}");

    // start from tail
    for i in (0..time).rev() {
        if calc(threshold, time, i) {
            tail = i;
            break;
        }
    }
    println!("tail: {tail}");

    let ans = tail - head + 1;
    println!("problem2 ans: {ans}");
}

fn calc(threshold: usize, time: usize, x: usize) -> bool {
    let dist = x * (time - x);
    dist > threshold
}
