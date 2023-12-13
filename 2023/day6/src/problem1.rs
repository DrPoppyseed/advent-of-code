use std::fs;

pub fn run() {
    let path = "inputs/2.txt";
    let mut lines = fs::read_to_string(path)
        .map(|c| c.lines().map(String::from).collect::<Vec<_>>())
        .unwrap()
        .into_iter();

    let mut arrs: Vec<Vec<usize>> = vec![];
    while let Some(line) = lines.next() {
        arrs.push(
            line.split(' ')
                .filter_map(|c| c.parse::<usize>().ok())
                .collect::<Vec<usize>>(),
        );
    }

    let (times, distances) = (&arrs[0], &arrs[1]);
    let mut ans: usize = 1;

    for i in 0..times.len() {
        let mut combinations = 0;
        for j in 0..times[i] {
            let dist = j * (times[i] - j);
            if dist > distances[i] {
                combinations += 1;
            }
        }

        ans *= combinations;
    }

    println!("problem1 ans: {ans}");
}
