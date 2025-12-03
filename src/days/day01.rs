#[allow(unused_variables)]
pub fn parse_input(input: &str) -> &str {
    input
}

#[allow(unused_variables)]
pub fn part1(input: &str) -> Option<u64> {
    let mut curr = 50;
    let mut counter: u64 = 0;

    for line in input.lines() {
        let mut chars = line.chars();

        let n = match chars.next() {
            Some('L') => -1,
            Some('R') => 1,
            _ => {
                panic!("invalid input");
            }
        };

        let mut x = chars.next().unwrap() as i32 - '0' as i32;

        for c in chars {
            let v = c as i32 - '0' as i32;
            x = x * 10 + v;
        }

        curr = (curr + n * x) % 100;

        if curr == 0 {
            counter += 1;
        }
    }

    Some(counter)
}

#[allow(unused_variables)]
pub fn part2(input: &str) -> Option<u64> {
    let mut curr = 50;
    let mut counter: u64 = 0;

    for line in input.lines() {
        let mut chars = line.chars();

        let n = match chars.next() {
            Some('L') => -1,
            Some('R') => 1,
            _ => {
                panic!("invalid input");
            }
        };

        let mut x = chars.next().unwrap() as i32 - '0' as i32;

        for c in chars {
            let v = c as i32 - '0' as i32;
            x = x * 10 + v;
        }

        x *= n;

        if x > 0 {
            curr += x;
            if curr > 99 {
                counter += (curr / 100) as u64;
                curr %= 100;
            }
        } else if x < 0 {
            let prev = curr;
            curr += x;

            let mut t = 0;
            if curr == 0 {
                t = 1;
            } else if prev == 0 {
                t = -(curr / 100) as u64;
            } else if prev > 0 && curr < 0 {
                t = (-(curr / 100) + 1) as u64;
            }

            counter += t;

            if curr < 0 {
                curr %= 100;
                if curr < 0 {
                    curr += 100;
                }
            }
        }
    }

    Some(counter)
}

#[cfg(test)]
pub mod tests {
    use super::*;
    const TEST_INPUT: &str = include_str!("../../input/day01/test.txt");
    #[test]
    fn test_day01_part1() {
        let input = parse_input(TEST_INPUT);

        let input = &input;
        let resp = part1(input);

        assert_eq!(resp, Some(6));
    }

    #[test]
    fn test_day01_part2() {
        let input = parse_input(TEST_INPUT);

        let input = &input;
        let resp = part2(input);

        assert_eq!(resp, Some(9));
    }
}
