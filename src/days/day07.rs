use std::collections::{HashMap, HashSet};

#[derive(Debug)]
pub struct Input {
    splitters: Vec<HashSet<usize>>, // indices
    starting_idx: usize,
}

pub fn parse_input(input: &str) -> Input {
    let mut splitters: Vec<HashSet<usize>> = Vec::new();
    let mut starting_idx = 0;
    for line in input.lines().step_by(2) {
        if starting_idx == 0 {
            starting_idx = line.len() / 2;
        }

        let splits = line
            .chars()
            .enumerate()
            .filter(|(_, c)| *c == '^')
            .map(|(i, _)| i)
            .collect();

        splitters.push(splits);
    }

    Input {
        splitters,
        starting_idx,
    }
}

#[allow(unused_variables)]
pub fn part1(input: &Input) -> Option<u64> {
    let mut count = 0;

    let mut curr: HashSet<usize> = HashSet::new();
    curr.insert(input.starting_idx);
    for sp in input.splitters.iter() {
        if sp.is_empty() {
            continue;
        }

        let mut new = HashSet::new();

        for el in &curr {
            if sp.contains(el) {
                count += 1;

                new.insert(el - 1);
                new.insert(el + 1);
            } else {
                new.insert(*el);
            }
        }

        curr = new;
    }

    Some(count)
}

#[allow(unused_variables)]
pub fn part2(input: &Input) -> Option<u64> {
    let mut curr: HashMap<usize, usize> = HashMap::new();
    curr.insert(input.starting_idx, 1);
    for sp in input.splitters.iter() {
        if sp.is_empty() {
            continue;
        }

        let mut new: HashMap<usize, usize> = HashMap::new();

        for (idx, beams) in curr {
            if sp.contains(&idx) {
                new.entry(idx - 1)
                    .and_modify(|e| *e += beams)
                    .or_insert(beams);
                new.entry(idx + 1)
                    .and_modify(|e| *e += beams)
                    .or_insert(beams);
            } else {
                new.entry(idx).and_modify(|e| *e += beams).or_insert(beams);
            }
        }

        let result: usize = new.values().sum();

        curr = new;
    }

    let result: usize = curr.values().sum();

    Some(result as u64)
}

#[cfg(test)]
pub mod tests {
    use super::*;
    const TEST_INPUT: &str = include_str!("../../input/day07/test.txt");
    #[test]
    fn test_day07_part1() {
        let input = parse_input(TEST_INPUT);

        let resp = part1(&input);

        assert_eq!(resp, Some(21));
    }

    #[test]
    fn test_day07_part2() {
        let input = parse_input(TEST_INPUT);

        let resp = part2(&input);

        assert_eq!(resp, Some(40));
    }
}
