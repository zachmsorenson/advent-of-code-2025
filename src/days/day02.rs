use num::Integer;

pub fn parse_input(input: &str) -> &str {
    input
}

#[allow(unused_variables)]
pub fn part1(input: &str) -> Option<u64> {
    let mut sum = 0;
    for range in input.trim_end().split(',') {
        let nums: Vec<&str> = range.split('-').collect();

        let low = nums[0].parse::<u64>();
        if low.is_err() {
            panic!("failed to parse: {}, {:?}", nums[0], low.err());
        }
        let low = low.unwrap();

        let high = nums[1].parse::<u64>();
        if high.is_err() {
            panic!("failed to parse: {}, {:?}", nums[1], high.err());
        }
        let high = high.unwrap();

        let ldigits = num_digits(low);
        let hdigits = num_digits(high);

        let mut curr = low;

        while curr <= high {
            let ndigits = num_digits(curr);
            if ndigits.is_odd() {
                curr += 1;
                continue;
            }

            let multiple = 10u64.pow(ndigits as u32 / 2);

            let l = curr / multiple;
            let r = curr % multiple;

            if l == r {
                sum += curr;
            }

            curr += 1;
        }
    }

    Some(sum)
}

#[allow(unused_variables)]
pub fn part2(input: &str) -> Option<u64> {
    let mut sum = 0;
    for range in input.trim_end().split(',') {
        let nums: Vec<&str> = range.split('-').collect();

        let low = nums[0].parse::<u64>();
        if low.is_err() {
            panic!("failed to parse: {}, {:?}", nums[0], low.err());
        }
        let low = low.unwrap();

        let high = nums[1].parse::<u64>();
        if high.is_err() {
            panic!("failed to parse: {}, {:?}", nums[1], high.err());
        }
        let high = high.unwrap();

        let mut curr = low;

        // println!("low: {}, high: {}, sum: {}", low, high, sum);

        while curr <= high {
            if is_invalid(curr) {
                // println!("\tfound invalid: {}", curr);
                sum += curr;
            }

            curr += 1;
        }
    }

    Some(sum)
}

fn num_digits(mut n: u64) -> u64 {
    let mut digits = 0;
    while n > 0 {
        digits += 1;
        n /= 10;
    }

    digits
}

fn is_invalid(num: u64) -> bool {
    let digits = num_digits(num);

    for chunk_size in 1..=digits / 2 {
        if !digits.is_multiple_of(chunk_size) {
            continue;
        }

        let num_chunks = digits / chunk_size;
        let val = nth_chunk(num, chunk_size as u32, 0);

        let mut found_invalid = true;
        for i in 1..num_chunks {
            if nth_chunk(num, chunk_size as u32, i) != val {
                found_invalid = false;
                break;
            }
        }

        if found_invalid {
            return true;
        }
    }

    false
}

fn nth_chunk(mut num: u64, chunk_size: u32, n: u64) -> u64 {
    let base = 10u64.pow(chunk_size);
    num /= base.pow(n as u32);

    num % base
}

#[cfg(test)]
pub mod tests {
    use super::*;
    const TEST_INPUT: &str = include_str!("../../input/day02/test.txt");
    #[test]
    fn test_day02_part1() {
        let input = parse_input(TEST_INPUT);

        let resp = part1(input);

        assert_eq!(resp, Some(1227775554));
    }

    #[test]
    fn test_day02_part2() {
        let input = parse_input(TEST_INPUT);

        let resp = part2(input);

        assert_eq!(resp, Some(4174379265));
    }
}
