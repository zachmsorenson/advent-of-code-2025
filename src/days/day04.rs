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
    let width = input.len as i32;
    let height = (total / input.len) as i32;

    let mut input = input.clone();
    let mut removable_coords = Vec::<(i32, i32)>::new();
    for i in 0..total {
        let cell = input.grid[i];
        if cell.c == '@' && cell.adj < 4 {
            let x0 = i as i32 % width;
            let y0 = i as i32 / width;

            removable_coords.push((x0, y0));
        }
    }

    while let Some((x0, y0)) = removable_coords.pop() {
        sum += 1;

        let i = y0 * width + x0;
        let cell = &mut input.grid[i as usize];
        cell.c = 'x';

        // update adjacent cell's adj count
        for (x1, y1) in DIRECTIONS {
            // inbounds
            let xj = x0 + x1;
            let yj = y0 + y1;

            if xj >= 0 && xj < width && yj >= 0 && yj < height {
                let j = ((yj) * width + (xj)) as usize;

                if input.grid[j].c == '@' && input.grid[j].adj > 0 {
                    input.grid[j].adj -= 1;
                    if input.grid[j].adj == 3 {
                        removable_coords.push((xj, yj));
                    }
                }
            }
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
