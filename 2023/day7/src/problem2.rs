use std::{cmp, fs};

#[derive(Debug, Clone)]
struct Hand {
    hand: [u8; 5],
    bid: usize,
    type_: u8,
}

impl Hand {
    fn new(chars: [char; 5], bid: usize) -> Hand {
        let hand = chars.map(|c| match c {
            'A' => 13,
            'K' => 12,
            'Q' => 11,
            'T' => 10,
            'J' => 1,
            c => (c as u32 - '0' as u32) as u8,
        });

        let mut sorted_hand = hand;
        sorted_hand.sort();

        let mut matches: Vec<u8> = vec![1];
        for i in 0..sorted_hand.len() - 1 {
            if sorted_hand[i] == sorted_hand[i + 1] && sorted_hand[i] != 1 {
                let len = matches.len() - 1;
                matches[len] += 1;
            } else {
                matches.push(1);
            }
        }
        matches.sort();

        let mut type_ = match &matches[..] {
            [5] => 7,
            [1, 4] => 6,
            [2, 3] => 5,
            [1, 1, 3] => 4,
            [1, 2, 2] => 3,
            [1, 1, 1, 2] => 2,
            _ => 1,
        };

        let mut joker_count = chars.into_iter().filter(|c| c == &'J').count();
        while joker_count > 0 && type_ < 7 {
            if type_ == 1 || type_ == 6 || type_ == 5 {
                type_ += 1;
            } else {
                type_ += 2;
            }
            joker_count -= 1;
        }

        Hand { hand, bid, type_ }
    }

    // I think we can assign a numeric value for all hands, without needing to
    // compare with other hands no?
    fn cmp(a: &Hand, b: &Hand) -> cmp::Ordering {
        if a.type_ == b.type_ {
            for i in 0..5 {
                if a.hand[i] > b.hand[i] {
                    return cmp::Ordering::Less;
                } else if a.hand[i] < b.hand[i] {
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

    println!("val: {val}");
}
