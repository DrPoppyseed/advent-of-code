use std::{
    collections::{HashMap, VecDeque},
    fs,
};

pub fn run() {
    let path = "inputs/3.txt";
    let content = fs::read_to_string(path).unwrap();
    let mut lines = content.lines();

    let mut commands = lines.next().unwrap().chars().collect::<VecDeque<char>>();
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

    println!("map: {map:?}");

    let mut dest = "AAA".to_string();
    let mut counter = 0;
    while let Some(command) = commands.pop_front() {
        commands.push_back(command);
        counter += 1;
        match command {
            'L' => {
                dest = map.get(&dest).unwrap()[0].clone();
            }
            'R' => {
                dest = map.get(&dest).unwrap()[1].clone();
            }
            _ => unreachable!(),
        }
        println!("dest: {dest}");
        if matches!(dest.as_str(), "ZZZ") {
            println!("counter: {counter}");
            break;
        }
    }
}
