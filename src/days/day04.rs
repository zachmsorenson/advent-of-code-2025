#[derive(Clone, Copy, Debug)]
pub struct Cell {
    c: char,
    adj: u64,
}

#[derive(Clone)]
pub struct Input {
    grid: Vec<Cell>,
    len: usize,
}

const DIRECTIONS: [(i32, i32); 8] = [
    (-1, -1),
    (-1, 0),
    (-1, 1),
    (0, -1),
    (0, 1),
    (1, -1),
    (1, 0),
    (1, 1),
];

impl Input {
    pub fn count_adj(&self, x0: i32, y0: i32) -> u64 {
        let mut count = 0;
        for (x1, y1) in DIRECTIONS {
            // println!("\t(x1, y1): ({}, {}), count: {}", x0 + x1, y0 + y1, count);

            if x0 + x1 < 0 || x0 + x1 >= self.len as i32 {
                continue;
            }

            if y0 + y1 < 0 || y0 + y1 >= self.grid.len() as i32 {
                continue;
            }

            let coord = (y0 + y1) * self.len as i32 + (x0 + x1);
            if 0 <= coord && coord < self.grid.len() as i32 && self.grid[coord as usize].c == '@' {
                count += 1;
            }
        }

        count
    }
}

pub fn parse_input(input: &str) -> Input {
    let mut grid = Vec::new();
    let mut len = 0;
    for line in input.lines() {
        if len == 0 {
            len = line.len();
        }

        for c in line.chars() {
            grid.push(Cell { c, adj: 0 });
        }
    }

    let mut parsed_input = Input { grid, len };

    for i in 0..len * input.lines().count() {
        let x = i % len;
        let y = i / len;

        let count = parsed_input.count_adj(x as i32, y as i32);
        parsed_input.grid[y * len + x].adj = count;
    }

    parsed_input
}

#[allow(unused_variables)]
pub fn part1(input: &Input) -> Option<u64> {
    let mut sum = 0;

    for cell in input.grid.iter() {
        if cell.c == '@' && cell.adj < 4 {
            sum += 1;
        }
    }

    Some(sum)
}

#[allow(unused_variables)]
pub fn part2(input: &Input) -> Option<u64> {
    let mut sum = 0;

    let total = input.grid.len();
    let width = input.len;
    let height = total / input.len;

    let mut input = input.clone();
    loop {
        let mut removable = 0;
        let mut i = 0;
        while i < total {
            let cell = &mut input.grid[i];
            if cell.c == '@' && cell.adj < 4 {
                removable += 1;
                cell.c = 'x';

                // OPT: update adjacent cell's adj count
                let x0 = i as i32 % width as i32;
                let y0 = i as i32 / width as i32;
                for (x1, y1) in DIRECTIONS {
                    if x0 + x1 >= 0
                        && x0 + x1 < width as i32
                        && y0 + y1 >= 0
                        && y0 + y1 < height as i32
                    {
                        let j = ((y0 + y1) * width as i32 + (x0 + x1)) as usize;
                        if input.grid[j].c == '@' && input.grid[j].adj > 0 {
                            input.grid[j].adj -= 1;
                        }
                    }
                }

                i = 0.max((i as i32 - width as i32 - 1) as usize);
            } else {
                i += 1
            }
        }

        sum += removable;

        if removable == 0 {
            break;
        }
    }

    Some(sum)
}

#[cfg(test)]
pub mod tests {
    use super::*;
    const TEST_INPUT: &str = include_str!("../../input/day04/test.txt");
    #[test]
    fn test_day04_part1() {
        let input = parse_input(TEST_INPUT);

        let resp = part1(&input);

        assert_eq!(resp, Some(13));
    }

    #[test]
    fn test_day04_part2() {
        let input = parse_input(TEST_INPUT);

        let resp = part2(&input);

        assert_eq!(resp, Some(43));
    }
}
