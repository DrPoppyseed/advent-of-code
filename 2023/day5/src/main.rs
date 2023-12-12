use std::{convert::Infallible, fs, str::FromStr};

fn main() {
    println!("problem1: {}", problem1("inputs/1.txt"));
    println!("problem2: {}", problem2("inputs/1.txt"));
}

#[derive(Default, Debug, Clone)]
struct Mapping {
    source: usize,
    dest: usize,
    length: usize,
}

impl Mapping {
    fn apply(&self, num: usize) -> Option<usize> {
        if num >= self.source && num <= (self.source + self.length) {
            let diff = num - self.source;
            Some(self.dest + diff)
        } else {
            None
        }
    }
}

impl FromStr for Mapping {
    type Err = Infallible;

    fn from_str(val: &str) -> Result<Self, Self::Err> {
        let [dest, source, length] = line_to_nums(val)[..3] else {
            unreachable!()
        };

        Ok(Mapping {
            source,
            dest,
            length,
        })
    }
}

#[derive(Default, Debug, Clone)]
struct AlmanacMap {
    rules: Vec<Mapping>,
}

impl AlmanacMap {
    fn map(&self, num: usize) -> usize {
        for rule in &self.rules {
            if let Some(location) = rule.apply(num) {
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
            let rule = Mapping::from_str(&line).unwrap();
            let last_idx = maps.len() - 1;
            maps[last_idx].rules.push(rule);
        } else {
            continue;
        }
    }

    (seeds, maps)
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

// note to self: from day5 on, try to think of problem1 and problem2 as
// completely different beasts. if you naievely try to reuse the same algorithm
// from problem1 for problem2, you'll see the same kind of horrors I saw with
// one.

#[derive(Debug)]
struct Remap {
    remap_left: usize,
    remap_right: usize,
    remap_diff: i32,
}

impl From<String> for Remap {
    fn from(val: String) -> Self {
        let [dest, source, length] = line_to_nums(&val)[..3] else {
            unreachable!()
        };

        Self {
            remap_left: source,
            remap_right: source + length - 1,
            remap_diff: dest as i32 - source as i32,
        }
    }
}

fn problem2(path: &str) -> usize {
    let mut lines = fs::read_to_string(path)
        .unwrap()
        .lines()
        .map(String::from)
        .collect::<Vec<_>>()
        .into_iter();

    let seeds = line_to_nums(&lines.next().unwrap());
    let mut seeds = (0..seeds.len())
        .into_iter()
        .step_by(2)
        .map(|i| (seeds[i], seeds[i] + seeds[i + 1] - 1))
        .collect::<Vec<(usize, usize)>>();

    lines.next();

    let mappings = |range| {

    };

    // let mut remapped_seeds: Vec<(usize, usize)> = vec![];
    // while let Some(line) = lines.next() {
    //     if line.contains("map") || line.is_empty() {
    //         continue;
    //     } else {
    //         let Remap {
    //             remap_left,
    //             remap_right,
    //             remap_diff,
    //         } = Remap::from(line.clone());
    //
    //         // find out if mapping overlaps with any of the seeds
    //         // if mapping overlaps with any, update for range
    //         // let mut seeds = seeds.clone();
    //         while !seeds.is_empty() {
    //             let (left, right) = seeds.pop().unwrap();
    //             if left <= remap_right && right >= remap_left {
    //                 // complete overlap. numbers in [max(remap_left, seed_left),
    //                 // min(remap_right, seed_right)] will be remapped
    //                 if left >= remap_left && right <= remap_right {
    //                     // seed range is inside remap range
    //                     println!("{left}-{right} inside");
    //                     remapped_seeds.push((
    //                         (left as i32 + remap_diff) as usize,
    //                         (right as i32 + remap_diff) as usize,
    //                     ));
    //                 } else if left <= remap_right && right >= remap_right {
    //                     // overlaps partially on seed's right
    //                     // append portion that overlaps (left, remap_right)
    //                     println!("{left}-{right} right");
    //                     remapped_seeds.push((
    //                         (left as i32 + remap_diff) as usize,
    //                         (remap_right as i32 + remap_diff) as usize,
    //                     ));
    //
    //                     // append portion that doesn't overlap (remap_right, right)
    //                     remapped_seeds.push((remap_right + 1, right));
    //                 } else if left <= remap_left && right >= remap_left {
    //                     // overlaps partially on seed's left
    //                     // append portion that overlaps (left, remap_left)
    //                     println!("{left}-{right} left");
    //                     remapped_seeds.push((left, remap_left));
    //
    //                     // append portion that doens't overlap (remap_left, right)
    //                     remapped_seeds.push((
    //                         (remap_left as i32 + remap_diff) as usize,
    //                         (right as i32 + remap_diff) as usize,
    //                     ));
    //                 } else {
    //                     // overlaps partially on both sides
    //                     // append portion that doesn't overlap on left (left, remap_left)
    //                     println!("{left}-{right}) both");
    //                     remapped_seeds.push((left, remap_left));
    //
    //                     // append portion that overlaps
    //                     remapped_seeds.push((
    //                         (remap_left as i32 + 1 + remap_diff) as usize,
    //                         (remap_right as i32 + remap_diff) as usize,
    //                     ));
    //
    //                     // append portion that doesn't overlap on right (remap_right, right)
    //                     remapped_seeds.push((remap_right + 1, right));
    //                 }
    //             } else {
    //                 remapped_seeds.push((left, right));
    //             }
    //         }
    //
    //         seeds.append(&mut remapped_seeds);
    //         seeds.sort();
    //         println!("remap: {remap_left}-{remap_right}, {remap_diff}   \t| seed: {seeds:?}");
    //     }
    // }
    //
    // seeds.into_iter().map(|(left, _)| left).min().unwrap()
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
