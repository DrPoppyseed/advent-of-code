use std::{
    collections::{HashMap, VecDeque},
    fs,
};

// We just need to find the LCM of all steps of all paths from A->Z
// Keep a vec of (key, steps) and pop paths found from search.
// Once all steps are found, just calculate LCM of all steps.
// This brings down the time from getting the LCM to finding the path with the
// most steps, which might be a big difference.
pub fn run() {
    let path = "inputs/3.txt";
    let content = fs::read_to_string(path).unwrap();
    let mut lines = content.lines();

    let commands = lines.next().unwrap().chars().collect::<VecDeque<char>>();
    lines.next();

    let mut map: HashMap<String, [String; 2]> = HashMap::new();
    while let Some(line) = lines.next() {
        let strs = line
            .split(|c: char| !c.is_ascii_alphabetic())
            .filter_map(|split| {
                if split.is_empty() {
                    None
                } else {
                    Some(String::from(split))
                }
            })
            .collect::<Vec<_>>();
        map.insert(strs[0].clone(), [strs[1].clone(), strs[2].clone()]);
    }

    let keys = map
        .keys()
        .filter(|k| matches!(k.chars().last(), Some('A')))
        .collect::<Vec<_>>();

    let mut positions = keys.into_iter().cloned().collect::<Vec<_>>();
    println!("starting positions: {positions:?}");

    let mut steps: Vec<usize> = vec![];
    let mut global_counter = 0;
    while !positions.is_empty() {
        let command = commands[global_counter % commands.len()];
        match command {
            'L' => {
                positions = positions
                    .iter()
                    .map(|p| map.get(p).unwrap()[0].clone())
                    .collect();
            }
            'R' => {
                positions = positions
                    .iter()
                    .map(|p| map.get(p).unwrap()[1].clone())
                    .collect();
            }
            _ => unreachable!(),
        }
        global_counter += 1;
        positions = positions
            .into_iter()
            .filter_map(|p| {
                if matches!(p.chars().last(), Some('Z')) {
                    steps.push(global_counter);
                    None
                } else {
                    Some(p)
                }
            })
            .collect();
    }

    println!("steps: {steps:?}");

    // find LCM
    let lcm = steps
        .into_iter()
        .fold(1u128, |acc, count| num::integer::lcm(acc, count as u128));

    println!("lcm: {lcm}");
}
