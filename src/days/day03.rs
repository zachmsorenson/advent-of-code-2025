pub fn parse_input(input: &str) -> &str {
    input
}

#[allow(unused_variables)]
pub fn part1(input: &str) -> Option<u64> {
    let mut sum = 0;
    for bank in input.lines() {
        let mut first = '0';
        let mut second = '0';

        for (i, c) in bank.chars().enumerate() {
            if i > 0 && c > second {
                second = c;
            }

            if i < bank.len() - 1 && c > first {
                first = c;
                second = '0';
            }
        }

        let val = 10 * (first as u64 - '0' as u64) + (second as u64 - '0' as u64);
        sum += val;
    }

    Some(sum)
}

#[allow(unused_variables)]
pub fn part2(input: &str) -> Option<u64> {
    let mut sum = 0;
    for bank in input.lines() {
        let mut values: [u8; 12] = [b'0'; 12];
        let chars: Vec<u8> = bank.chars().map(|c| c as u8).collect();

        let mut left = 0;
        let mut right = bank.len() - 11;

        for (i, v) in values.iter_mut().enumerate() {
            let mut idx = left;
            let mut max = b'0';

            (left..right).for_each(|j| {
                let c = chars[j];
                if c > max {
                    max = c;
                    idx = j;
                }
            });

            left = idx + 1;
            right += 1;
            *v = max;
        }

        for (i, &v) in values.iter().enumerate() {
            sum += 10u64.pow(11 - i as u32) * (v as u64 - '0' as u64);
        }
    }

    Some(sum)
}

#[cfg(test)]
pub mod tests {
    use super::*;
    const TEST_INPUT: &str = include_str!("../../input/day03/test.txt");
    #[test]
    fn test_day03_part1() {
        let input = parse_input(TEST_INPUT);

        let resp = part1(input);

        assert_eq!(resp, Some(357));
    }

    #[test]
    fn test_day03_part2() {
        let input = parse_input(TEST_INPUT);

        let resp = part2(input);

        assert_eq!(resp, Some(3121910778619));
    }
}
