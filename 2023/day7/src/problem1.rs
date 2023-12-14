use std::{cmp, fs};

#[derive(Debug, Clone)]
struct Hand {
    hand: [char; 5],
    strengths: [u8; 5],
    bid: usize,
    type_: u8,
}

impl Hand {
    fn new(hand: [char; 5], bid: usize) -> Hand {
        let strengths = hand.map(|c| match c {
            'A' => 14,
            'K' => 13,
            'Q' => 12,
            'J' => 11,
            'T' => 10,
            c => (c as u32 - '0' as u32) as u8,
        });

        let mut sorted_strengths = strengths;
        sorted_strengths.sort();
        println!("sorted_strengths: {sorted_strengths:?}");

        let mut matches: Vec<u8> = vec![1];
        for i in 0..sorted_strengths.len() - 1 {
            if sorted_strengths[i] == sorted_strengths[i + 1] {
                let len = matches.len() - 1;
                matches[len] += 1;
            } else {
                matches.push(1);
            }
        }
        matches.sort();
        println!("matches: {matches:?}");

        let type_ = match &matches[..] {
            [5] => 7,
            [1, 4] => 6,
            [2, 3] => 5,
            [1, 1, 3] => 4,
            [1, 2, 2] => 3,
            [1, 1, 1, 2] => 2,
            _ => 1,
        };

        Hand {
            hand,
            strengths,
            bid,
            type_,
        }
    }

    // I think we can assign a numeric value for all hands, without needing to
    // compare with other hands no?
    fn cmp(a: &Hand, b: &Hand) -> cmp::Ordering {
        if a.type_ == b.type_ {
            for i in 0..5 {
                if a.strengths[i] > b.strengths[i] {
                    return cmp::Ordering::Less;
                } else if a.strengths[i] < b.strengths[i] {
                    return cmp::Ordering::Greater;
                }
            }
            return cmp::Ordering::Equal;
        } else if a.type_ > b.type_ {
            cmp::Ordering::Less
        } else {
            cmp::Ordering::Greater
        }
    }
}

pub fn run() {
    let path = "inputs/2.txt";
    let mut hands: Vec<Hand> = fs::read_to_string(path)
        .unwrap()
        .lines()
        .map(|l| {
            let strs = l.split(' ').collect::<Vec<_>>();
            let mut hand: [char; 5] = ['0'; 5];
            strs[0].chars().enumerate().for_each(|(i, c)| {
                hand[i] = c;
            });
            Hand::new(hand, strs[1].parse::<usize>().unwrap())
        })
        .collect::<Vec<_>>();

    hands.sort_by(|a, b| Hand::cmp(a, b));

    let mut val = 0;
    for (i, hand) in hands.iter().rev().enumerate() {
        val += hand.bid * (i + 1);
    }

    println!("hands: {hands:?}");
    println!("val: {val}");
}
