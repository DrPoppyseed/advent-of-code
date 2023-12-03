use std::{cell::RefCell, collections::VecDeque, fs, str};

fn main() {
    println!("problem1: {}", problem1("inputs/2.txt"));
    println!("problem2: {}", problem2("inputs/2.txt"));
}

// chars in head->tail become a numeric string
#[derive(Clone, Debug)]
struct RawPart {
    head: (usize, usize), // (row, col) coord of beginning
    tail: usize,          // col coord of tail
}

impl RawPart {
    fn to_num(&self, chars: &Vec<Vec<char>>) -> usize {
        let mut builder = String::new();
        for col in self.head.1..self.tail + 1 {
            builder += str::from_utf8(&[chars[self.head.0][col] as u8]).unwrap();
        }
        builder.parse::<usize>().unwrap()
    }
}

struct Dimensions {
    rows: usize,
    cols: usize,
}

fn make_grid(path: &str) -> (Dimensions, Vec<Vec<char>>) {
    let lines = fs::read_to_string(path)
        .map(|c| c.lines().map(String::from).collect::<Vec<_>>())
        .unwrap();

    let dimensions = Dimensions {
        rows: lines.len(),
        cols: lines.first().unwrap().len(),
    };

    let grid = lines
        .clone()
        .iter()
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect::<Vec<Vec<_>>>();

    (dimensions, grid)
}

fn problem1(path: &str) -> usize {
    let (Dimensions { rows, cols }, grid) = make_grid(path);
    let visited: RefCell<Vec<(usize, usize)>> = RefCell::new(Vec::new());
    let raw_parts: RefCell<Vec<RawPart>> = RefCell::new(Vec::new());

    // creates number by exploring numeric cells on the same row
    let explore_digit = |raw_part: RawPart, row: usize, col: usize| {
        let mut raw_part = raw_part.clone();
        let mut deq = VecDeque::from([(row, col)]);

        while let Some((drow, dcol)) = deq.pop_front() {
            if visited.borrow().contains(&(drow, dcol)) {
                continue;
            }

            visited.borrow_mut().push((drow, dcol));
            if dcol > raw_part.tail {
                raw_part.tail = dcol;
            }
            if dcol < raw_part.head.1 {
                raw_part.head = (raw_part.head.0, dcol);
            }

            if (dcol as i16) - 1 >= 0 && grid[drow][dcol - 1].is_ascii_digit() {
                deq.push_back((drow, dcol - 1));
            }

            if dcol + 1 < cols && grid[drow][dcol + 1].is_ascii_digit() {
                deq.push_back((drow, dcol + 1))
            }
        }

        raw_parts.borrow_mut().push(raw_part);
    };

    // explores the area surrounding symbols
    let explore = |row: usize, col: usize| {
        let coords = [
            (row + 1, col),
            (row - 1, col),
            (row, col + 1),
            (row, col - 1),
            (row + 1, col + 1),
            (row + 1, col - 1),
            (row - 1, col + 1),
            (row - 1, col - 1),
        ];

        for (crow, ccol) in coords {
            // if out of bounds, continue
            if !(0..rows).contains(&crow)
                || !(0..cols).contains(&ccol)
                || visited.borrow().contains(&(crow, ccol))
            {
                continue;
            }

            let cc = grid[crow][ccol];

            // if not number, visit but continue
            if cc == '.' {
                let mut c_visited = visited.clone().into_inner();
                c_visited.push((crow, ccol));
                continue;
            }

            if cc.is_ascii_digit() {
                let raw_part = RawPart {
                    head: (crow, ccol),
                    tail: ccol,
                };
                explore_digit(raw_part, crow, ccol);
            }
        }
    };

    for row in 0..rows {
        for col in 0..cols {
            let c = &grid[row][col];
            if !visited.borrow().contains(&(row, col)) && !c.is_ascii_digit() && c != &'.' {
                // explore neighbors
                explore(row, col);
            }
        }
    }

    let sum: usize = raw_parts
        .into_inner()
        .into_iter()
        .map(|raw_part| raw_part.to_num(&grid))
        .sum();

    sum
}

fn problem2(path: &str) -> usize {
    let (Dimensions { rows, cols }, grid) = make_grid(path);
    let visited: RefCell<Vec<(usize, usize)>> = RefCell::new(Vec::new());

    // creates number by exploring numeric cells on the same row
    let explore_digit = |raw_part: RawPart, row: usize, col: usize| -> RawPart {
        let mut raw_part = raw_part.clone();
        let mut deq = VecDeque::from([(row, col)]);

        while let Some((drow, dcol)) = deq.pop_front() {
            if visited.borrow().contains(&(drow, dcol)) {
                continue;
            }

            visited.borrow_mut().push((drow, dcol));
            if dcol > raw_part.tail {
                raw_part.tail = dcol;
            }
            if dcol < raw_part.head.1 {
                raw_part.head = (raw_part.head.0, dcol);
            }

            if (dcol as i16) - 1 >= 0 && grid[drow][dcol - 1].is_ascii_digit() {
                deq.push_back((drow, dcol - 1));
            }

            if dcol + 1 < cols && grid[drow][dcol + 1].is_ascii_digit() {
                deq.push_back((drow, dcol + 1))
            }
        }

        raw_part
    };

    // explores the area surrounding symbols
    let explore = |row: usize, col: usize| -> Vec<RawPart> {
        let mut raw_parts = Vec::new();
        let coords = [
            (row + 1, col),
            (row - 1, col),
            (row, col + 1),
            (row, col - 1),
            (row + 1, col + 1),
            (row + 1, col - 1),
            (row - 1, col + 1),
            (row - 1, col - 1),
        ];

        for (crow, ccol) in coords {
            // if out of bounds, continue
            if !(0..rows).contains(&crow)
                || !(0..cols).contains(&ccol)
                || visited.borrow().contains(&(crow, ccol))
            {
                continue;
            }

            let cc = grid[crow][ccol];

            // if not number, visit but continue
            if cc == '.' {
                let mut c_visited = visited.clone().into_inner();
                c_visited.push((crow, ccol));
                continue;
            }

            if cc.is_ascii_digit() {
                let raw_part = RawPart {
                    head: (crow, ccol),
                    tail: ccol,
                };
                raw_parts.push(explore_digit(raw_part, crow, ccol));
            }
        }

        raw_parts
    };

    let mut gear_ratio = 0;

    for row in 0..rows {
        for col in 0..cols {
            let c = &grid[row][col];
            if !visited.borrow().contains(&(row, col)) && c == &'*' {
                // explore neighbors
                let gear_adj = explore(row, col);
                if gear_adj.len() == 2 {
                    gear_ratio += gear_adj
                        .into_iter()
                        .map(|raw_part| raw_part.to_num(&grid))
                        .fold(1, |acc, num| acc * num);
                }
            }
        }
    }

    gear_ratio
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn problem1_input1() {
        let sum = problem1("inputs/1.txt");
        assert_eq!(sum, 4361);
    }

    #[test]
    fn problem2_input1() {
        let ratio = problem2("inputs/1.txt");
        assert_eq!(ratio, 467835);
    }
}
