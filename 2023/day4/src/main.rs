use std::{convert::Infallible, fs, str::FromStr};

fn main() {
    println!("problem1: {}", problem1("inputs/2.txt"));
    println!("problem2: {}", problem2("inputs/2.txt"));
}

#[derive(Debug, Clone)]
struct Card {
    owned_nums: Vec<usize>,
    winning_nums: Vec<usize>,
}

impl Card {
    fn calc1(&self) -> usize {
        match self.matches() {
            0 => 0,
            1 => 1,
            owned => 2usize.pow(owned as u32 - 1),
        }
    }

    fn clean(nums: &str) -> Vec<usize> {
        nums.trim()
            .split(' ')
            .filter(|c| !c.is_empty())
            .map(|num| num.parse::<usize>().unwrap())
            .collect::<Vec<_>>()
    }

    fn matches(&self) -> usize {
        self.owned_nums
            .iter()
            .filter(|num| self.winning_nums.contains(num))
            .count()
    }
}

impl FromStr for Card {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let rest = s.split(':').collect::<Vec<_>>()[1];
        if let [winning_nums, owned_nums] = rest.split('|').collect::<Vec<_>>()[..2] {
            let winning_nums = Card::clean(winning_nums);
            let owned_nums = Card::clean(owned_nums);

            Ok(Card {
                owned_nums,
                winning_nums,
            })
        } else {
            unreachable!()
        }
    }
}

fn problem1(path: &str) -> usize {
    fs::read_to_string(path)
        .map(|content| {
            content
                .lines()
                .map(|line| Card::from_str(line).unwrap().calc1())
                .sum()
        })
        .unwrap()
}

fn problem2(path: &str) -> usize {
    let cards = fs::read_to_string(path)
        .map(|content| {
            content
                .lines()
                .map(|line| Card::from_str(line).unwrap())
                .collect::<Vec<_>>()
        })
        .unwrap();

    let mut copies = vec![1; cards.len()];
    cards.iter().enumerate().for_each(|(i, card)| {
        (i + 1..i + card.matches() + 1).into_iter().for_each(|j| {
            copies[j] += copies[i];
        });
    });

    copies.iter().sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn problem1_input1() {
        assert_eq!(problem1("inputs/1.txt"), 13)
    }

    #[test]
    fn problem1_input2() {
        assert_eq!(problem1("inputs/2.txt"), 32001)
    }

    #[test]
    fn problem2_input1() {
        assert_eq!(problem2("inputs/1.txt"), 30)
    }
}
