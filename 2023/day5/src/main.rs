use rayon::prelude::*;
use std::{convert::Infallible, fs, str::FromStr};

fn main() {
    // println!("problem1: {}", problem1("inputs/2.txt"));
    println!("problem2: {}", problem2("inputs/2.txt"));
}

#[derive(Default, Debug, Clone)]
struct Rule {
    source: usize,
    dest: usize,
    length: usize,
}

impl Rule {
    fn apply_rule(&self, num: usize) -> Option<usize> {
        if num >= self.source && num <= (self.source + self.length) {
            let diff = num - self.source;
            Some(self.dest + diff)
        } else {
            None
        }
    }
}

impl FromStr for Rule {
    type Err = Infallible;

    fn from_str(val: &str) -> Result<Self, Self::Err> {
        let [dest, source, length] = line_to_nums(val)[..3] else {
            unreachable!()
        };

        Ok(Rule {
            source,
            dest,
            length,
        })
    }
}

// This or the `name` could be an enum, but I'll keep it as a String until I learn
// more about problem2
#[derive(Default, Debug, Clone)]
struct AlmanacMap {
    rules: Vec<Rule>,
}

impl AlmanacMap {
    fn map(&self, num: usize) -> usize {
        for rule in &self.rules {
            if let Some(location) = rule.apply_rule(num) {
                return location;
            }
        }
        num
    }
}

fn line_to_nums(line: &str) -> Vec<usize> {
    line.split(' ')
        .filter_map(|el| el.parse::<usize>().ok())
        .collect::<Vec<usize>>()
}

fn make_maps(path: &str) -> (Vec<usize>, Vec<AlmanacMap>) {
    let lines: Vec<String> = fs::read_to_string(path)
        .map(|c| c.lines().map(String::from).collect())
        .unwrap();

    let mut lines = lines.into_iter();
    let seeds = line_to_nums(&lines.next().unwrap());
    lines.next();

    let mut maps: Vec<AlmanacMap> = vec![];
    while let Some(line) = lines.next() {
        if line.contains("map") {
            maps.push(AlmanacMap::default())
        } else if !line.is_empty() {
            let rule = Rule::from_str(&line).unwrap();
            let last_idx = maps.len() - 1;
            maps[last_idx].rules.push(rule);
        } else {
            continue;
        }
    }

    (seeds, maps)
}

#[derive(Debug)]
struct SeedPair {
    floor: usize,
    range: usize,
}

fn problem1(path: &str) -> usize {
    let (seeds, maps) = make_maps(path);

    let mut locations: Vec<usize> = vec![];
    for seed in seeds {
        let mut location = seed;
        for map in &maps {
            location = map.map(location);
        }
        locations.push(location);
    }

    locations.into_iter().min().unwrap()
}

fn make_seed_pairs(nums: Vec<usize>) -> Vec<SeedPair> {
    let mut pairs: Vec<SeedPair> = vec![];
    for (i, num) in nums.iter().step_by(2).enumerate() {
        pairs.push(SeedPair {
            floor: *num,
            range: nums[i * 2 + 1],
        })
    }
    pairs
}

fn problem2(path: &str) -> usize {
    let (nums, maps) = make_maps(path);
    let sps = make_seed_pairs(nums);

    sps.into_par_iter()
        .map(|sp| {
            let min = (sp.floor..sp.floor + sp.range + 1)
                .into_par_iter()
                .map(|seed| {
                    let mut location: usize = seed;
                    maps.clone().iter().for_each(|map| {
                        location = map.map(location);
                    });

                    location
                })
                .min()
                .unwrap();

            min
        })
        .min()
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn problem1_input1() {
        assert_eq!(problem1("inputs/1.txt"), 35);
    }

    #[test]
    fn problem1_input2() {
        assert_eq!(problem1("inputs/2.txt"), 318728750);
    }

    #[test]
    fn problem2_input1() {
        assert_eq!(problem2("inputs/1.txt"), 46);
    }
}
