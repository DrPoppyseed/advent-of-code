use std::{cmp, fs};

fn main() {
    problem1("inputs/2.txt");
    problem2("inputs/2.txt");
}

#[derive(Debug)]
struct Game {
    id: usize,
    red: usize,
    green: usize,
    blue: usize,
}

impl Game {
    const MAX_RED: usize = 12;
    const MAX_GREEN: usize = 13;
    const MAX_BLUE: usize = 14;

    fn possible(&self) -> bool {
        self.red <= Self::MAX_RED && self.green <= Self::MAX_GREEN && self.blue <= Self::MAX_BLUE
    }

    fn power(&self) -> usize {
        self.red * self.green * self.blue
    }
}

#[derive(Debug)]
enum Ball {
    Red(usize),
    Green(usize),
    Blue(usize),
}

impl Ball {
    fn from_str(input: &str) -> Self {
        let [count, color] = input.trim().split(' ').collect::<Vec<&str>>().as_slice()[..2] else {
            panic!("wat");
        };
        let count = count.parse::<usize>().unwrap();

        match color {
            "blue" => Self::Blue(count),
            "red" => Self::Red(count),
            "green" => Self::Green(count),
            _ => panic!("wat"),
        }
    }
}

fn txt_to_games(path: &str) -> Vec<Game> {
    fs::read_to_string(path)
        .map(|txt| txt.lines().map(String::from).collect::<Vec<_>>())
        .unwrap()
        .into_iter()
        .map(move |line| {
            let [id, rest]: &[&str] = &line.split(':').collect::<Vec<&str>>()[..2] else {
                panic!("wat");
            };
            let id = id.strip_prefix("Game ").unwrap().parse::<usize>().unwrap();

            let mut max_red = 0;
            let mut max_green = 0;
            let mut max_blue = 0;
            let _ = rest
                .split([';', ','])
                .map(Ball::from_str)
                .for_each(|ball| match ball {
                    Ball::Red(c) => {
                        max_red = cmp::max(max_red, c);
                    }
                    Ball::Green(c) => {
                        max_green = cmp::max(max_green, c);
                    }
                    Ball::Blue(c) => {
                        max_blue = cmp::max(max_blue, c);
                    }
                });

            let game = Game {
                id,
                red: max_red,
                green: max_green,
                blue: max_blue,
            };

            game
        })
        .collect::<Vec<_>>()
}

fn problem1(path: &str) {
    let total = txt_to_games(path)
        .into_iter()
        .filter(|game| game.possible())
        .fold(0, |acc, game| acc + game.id);

    println!("total: {total}");
}

fn problem2(path: &str) {
    let powered: usize = txt_to_games(path)
        .into_iter()
        .map(|game| game.power())
        .sum();

    println!("powered: {powered}");
}
