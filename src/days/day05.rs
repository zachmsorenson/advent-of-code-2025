pub struct Input {
    ranges: Vec<(u64, u64)>,
    ingredients: Vec<u64>,
}

pub fn parse_input(input: &str) -> Input {
    let mut init_ranges = Vec::new();
    let mut ingredients = Vec::new();

    for line in input.lines() {
        let mut sp = line.split('-');

        let first = match sp.next() {
            Some("") => continue,
            None => continue,
            Some(s) => s.parse().unwrap(),
        };

        match sp.next() {
            Some(s) => {
                let second: u64 = s.parse().unwrap();
                init_ranges.push((first, second))
            }
            None => ingredients.push(first),
        }
    }

    let mut ranges: Vec<(u64, u64)> = Vec::new();
    init_ranges.sort();

    let mut i = 0;
    let (mut curr_low, mut curr_high) = init_ranges[0];
    i += 1;
    while i < init_ranges.len() {
        let (next_low, next_high) = init_ranges[i];

        // plus one because adjacent values can be combined to one range
        if next_low > curr_high + 1 {
            // discontinuity, write the current range
            ranges.push((curr_low, curr_high));
            (curr_low, curr_high) = (next_low, next_high);
        } else {
            // overlap with current range
            curr_high = curr_high.max(next_high);
        }

        i += 1;
    }

    ranges.push((curr_low, curr_high));

    ingredients.sort();

    Input {
        ranges,
        ingredients,
    }
}

#[allow(unused_variables)]
pub fn part1(input: &Input) -> Option<u64> {
    let mut count = 0;

    let mut i = 0;
    for ingr in &input.ingredients {
        while i < input.ranges.len() {
            let (low, high) = input.ranges[i];

            if low <= *ingr && *ingr <= high {
                // ingredient is in current range, count and move on
                count += 1;
                break;
            } else if *ingr < low {
                // ingredient is in gap, not counted
                break;
            } else {
                // go to next range
                i += 1
            }
        }
    }

    Some(count)
}

#[allow(unused_variables)]
pub fn part2(input: &Input) -> Option<u64> {
    let mut count = 0;
    for (low, high) in &input.ranges {
        count += high - low + 1;
    }

    Some(count)
}

#[cfg(test)]
pub mod tests {
    use super::*;
    const TEST_INPUT: &str = include_str!("../../input/day05/test.txt");
    #[test]
    fn test_day05_part1() {
        let input = parse_input(TEST_INPUT);

        let resp = part1(&input);

        assert_eq!(resp, Some(3));
    }

    #[test]
    fn test_day05_part2() {
        let input = parse_input(TEST_INPUT);

        let resp = part2(&input);

        assert_eq!(resp, Some(14));
    }
}
