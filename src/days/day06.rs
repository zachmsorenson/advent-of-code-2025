#[derive(Debug)]
pub struct Problem {
    nums: Vec<u64>,
    operator: char,
}

impl Problem {
    pub fn result(self) -> u64 {
        self.nums
            .into_iter()
            .reduce(|acc, n| match self.operator {
                '+' => acc + n,
                '*' => acc * n,
                _ => acc,
            })
            .unwrap()
    }
}

pub fn parse_input(input: &str) -> &str {
    input
}

#[allow(unused_variables)]
pub fn part1(input: &str) -> Option<u64> {
    let chars: Vec<Vec<char>> = input.lines().map(|l| l.chars().collect()).collect();
    let operators = chars
        .last()
        .unwrap()
        .iter()
        .filter(|&c| *c == '+' || *c == '*');

    let mut problems: Vec<Problem> = operators
        .map(|&op| Problem {
            nums: Vec::new(),
            operator: op,
        })
        .collect();

    let width = chars[0].len();
    let height = chars.len() - 1;

    for line in input.lines().take(height) {
        for (k, sp) in line.split_whitespace().enumerate() {
            let num = sp.parse().unwrap();
            problems[k].nums.push(num);
        }
    }

    let mut sum = 0;
    for p in problems {
        sum += p.result();
    }

    Some(sum)
}

#[allow(unused_variables)]
pub fn part2(input: &str) -> Option<u64> {
    let chars: Vec<Vec<char>> = input.lines().map(|l| l.chars().collect()).collect();
    let operators = chars
        .last()
        .unwrap()
        .iter()
        .filter(|&c| *c == '+' || *c == '*');

    let mut problems: Vec<Problem> = operators
        .map(|&op| Problem {
            nums: Vec::new(),
            operator: op,
        })
        .collect();

    let width = chars[0].len();
    let height = chars.len() - 1;

    let mut k = 0;
    for i in 0..width {
        let mut digits = String::new();
        for j in 0..height {
            let c = chars[j][i];
            if let '0'..='9' = chars[j][i] {
                digits.push(c)
            }
        }

        if digits.is_empty() {
            k += 1;
            continue;
        }

        let n = digits.parse().unwrap();
        problems[k].nums.push(n)
    }

    let mut sum = 0;
    for p in problems {
        sum += p.result();
    }
    Some(sum)
}

#[cfg(test)]
pub mod tests {
    use super::*;
    const TEST_INPUT: &str = include_str!("../../input/day06/test.txt");
    #[test]
    fn test_day06_part1() {
        let input = parse_input(TEST_INPUT);

        let resp = part1(input);

        assert_eq!(resp, Some(4277556));
    }

    #[test]
    fn test_day06_part2() {
        let input = parse_input(TEST_INPUT);

        let resp = part2(input);

        assert_eq!(resp, Some(3263827));
    }
}
