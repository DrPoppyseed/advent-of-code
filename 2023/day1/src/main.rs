#![allow(dead_code)]

use std::fs;

use regex::Regex;

fn main() {
    // problem1("./inputs/input2.txt");
    problem2("./inputs/input4.txt");
}

/// solution for problem1
fn problem1(path: &str) {
    let lines = fs::read_to_string(path)
        .map(|content| content.lines().map(String::from).collect::<Vec<_>>())
        .unwrap();

    let sum = lines
        .into_iter()
        .map(|line| {
            let first = line
                .chars()
                .nth(line.find(|c| char::is_digit(c, 10)).unwrap())
                .and_then(|c| c.to_digit(10))
                .unwrap();
            let last = line
                .chars()
                .nth(line.rfind(|c| char::is_digit(c, 10)).unwrap())
                .and_then(|c| c.to_digit(10))
                .unwrap();

            (first, last)
        })
        .fold(0, |acc, (first, last)| acc + 10 * first + last);

    println!("{sum}");
}

/// solution for problem2
enum Rep {
    Digit(char),
    Word(String),
}

impl Rep {
    fn to_digit(&self) -> usize {
        match self {
            Rep::Digit(c) => c.to_digit(10).unwrap() as usize,
            Rep::Word(word) => match word.as_str() {
                "one" => 1,
                "two" => 2,
                "three" => 3,
                "four" => 4,
                "five" => 5,
                "six" => 6,
                "seven" => 7,
                "eight" => 8,
                "nine" => 9,
                _ => panic!("wat"),
            },
        }
    }

    fn as_digit(line: &str, idx: usize) -> Self {
        Rep::Digit(line.chars().nth(idx).unwrap())
    }
}

fn problem2(path: &str) -> usize {
    let lines = fs::read_to_string(path)
        .map(|content| content.lines().map(String::from).collect::<Vec<_>>())
        .unwrap();

    let re = Regex::new(r"one|two|three|four|five|six|seven|eight|nine").unwrap();
    let rre = Regex::new(r"eno|owt|eerht|ruof|evif|xis|neves|thgie|enin").unwrap();
    let is_digit = |c: char| char::is_digit(c, 10);

    let sum = lines
        .into_iter()
        .map(|line| {
            let first = {
                let idx = line.find(is_digit);
                let m = re.find(&line);

                match (idx, m) {
                    (Some(idx), Some(m)) => {
                        if idx < m.start() {
                            Rep::as_digit(&line, idx)
                        } else {
                            Rep::Word(m.as_str().to_string())
                        }
                    }
                    (Some(idx), None) => Rep::as_digit(&line, idx),
                    (None, Some(idx)) => Rep::Word(idx.as_str().to_string()),
                    _ => panic!("wat"),
                }
                .to_digit()
            };

            let last = {
                let idx = line.rfind(is_digit);
                let rline = line.chars().rev().collect::<String>();
                let m = rre.find(&rline);

                match (idx, m) {
                    (Some(idx), Some(m)) => {
                        if idx > (rline.len() - m.end()) {
                            Rep::as_digit(&line, idx)
                        } else {
                            Rep::Word(m.as_str().chars().rev().collect::<String>())
                        }
                    }
                    (Some(idx), None) => Rep::as_digit(&line, idx),
                    (None, Some(m)) => {
                        let reversed = m.as_str().chars().rev().collect::<String>();
                        Rep::Word(reversed)
                    }
                    _ => panic!("wat"),
                }
                .to_digit()
            };

            // println!("{first}, {last}");
            (first, last)
        })
        .fold(0, |acc, (first, last)| acc + 10 * first + last);

    println!("{sum}");
    sum
}

#[cfg(test)]
mod tests {
    use crate::problem2;

    #[test]
    fn test_input3() {
        assert_eq!(281, problem2("./inputs/input3.txt"));
    }

    #[test]
    fn test_input5() {
        assert_eq!(174, problem2("./inputs/input5.txt"));
    }
}
