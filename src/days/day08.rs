use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashMap, HashSet},
};

#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Copy, Clone)]
pub struct Point {
    x: i64,
    y: i64,
    z: i64,
}

pub struct Input {
    coords: Vec<Point>,
    connections: u32,
}

impl Point {
    // squared distance, we don't need the real thing
    fn dist(&self, other: &Point) -> i64 {
        (self.x - other.x) * (self.x - other.x)
            + (self.y - other.y) * (self.y - other.y)
            + (self.z - other.z) * (self.z - other.z)
    }
}

pub fn parse_input(input: &str) -> Input {
    let mut lines = input.lines();

    let connections = lines.next().unwrap().parse().unwrap();

    // skip empty newline - I modified my input to include the number of connections
    lines.next();

    let mut coords: Vec<Point> = Vec::new();
    for line in lines {
        let mut sp = line.split(',');
        let x = sp.next().unwrap().parse().unwrap();
        let y = sp.next().unwrap().parse().unwrap();
        let z = sp.next().unwrap().parse().unwrap();
        coords.push(Point { x, y, z });
    }

    Input {
        coords,
        connections,
    }
}

#[derive(Ord, Eq, PartialEq, PartialOrd, Debug)]
struct HeapElement {
    distance: i64,
    points: (usize, usize),
    debug_points: (Point, Point),
}

#[allow(unused_variables)]
pub fn part1(input: &Input) -> Option<u64> {
    // key: idx tuple in coords Vec (in ascending order), value: distance between those points
    let mut heap = BinaryHeap::new();

    for i in 0..input.coords.len() {
        let left = &input.coords[i];
        for j in i + 1..input.coords.len() {
            let right = &input.coords[j];

            let el = HeapElement {
                distance: left.dist(right),
                points: (i, j),
                debug_points: (*left, *right),
            };

            heap.push(Reverse(el));
        }
    }

    let mut circuits: Vec<HashSet<usize>> = Vec::new();
    let mut i = 0;
    while let Some(el) = heap.pop() {
        if i == input.connections as usize {
            break;
        }

        let mut new_vec: Vec<HashSet<usize>> = Vec::new();
        let mut new = HashSet::new();
        new.insert(el.0.points.0);
        new.insert(el.0.points.1);

        for circ in &circuits {
            if circ.contains(&el.0.points.0) || circ.contains(&el.0.points.1) {
                new.extend(circ);
            } else {
                new_vec.push(circ.clone());
            }
        }

        new_vec.push(new);
        circuits = new_vec;

        i += 1;
    }

    let mut counts: Vec<usize> = circuits.iter().map(|set| set.len()).collect();
    counts.sort();
    let result: usize = counts.iter().rev().take(3).product();

    // println!("circuits: {:?}", circuits);

    Some(result as u64)
}

#[allow(unused_variables)]
pub fn part2(input: &Input) -> Option<u64> {
    // key: idx tuple in coords Vec (in ascending order), value: distance between those points
    let mut heap = BinaryHeap::new();

    for i in 0..input.coords.len() {
        let left = &input.coords[i];
        for j in i + 1..input.coords.len() {
            let right = &input.coords[j];

            let el = HeapElement {
                distance: left.dist(right),
                points: (i, j),
                debug_points: (*left, *right),
            };

            heap.push(Reverse(el));
        }
    }

    // println!("heap: {:?}", heap);

    let mut circuits: Vec<HashSet<usize>> = Vec::new();
    let mut i = 0;
    let mut left = 0;
    let mut right = 0;
    while let Some(el) = heap.pop() {
        let mut new_vec: Vec<HashSet<usize>> = Vec::new();
        let mut new = HashSet::new();
        new.insert(el.0.points.0);
        new.insert(el.0.points.1);

        for circ in &circuits {
            if circ.contains(&el.0.points.0) || circ.contains(&el.0.points.1) {
                new.extend(circ);
            } else {
                new_vec.push(circ.clone());
            }
        }

        if new.len() == input.coords.len() {
            left = el.0.points.0;
            right = el.0.points.1;
            break;
        }

        new_vec.push(new);
        circuits = new_vec;

        i += 1;
    }

    let left = input.coords[left];
    let right = input.coords[right];

    let result = left.x * right.x;

    Some(result as u64)
}

#[cfg(test)]
pub mod tests {
    use super::*;
    const TEST_INPUT: &str = include_str!("../../input/day08/test.txt");
    #[test]
    fn test_day08_part1() {
        let input = parse_input(TEST_INPUT);

        let resp = part1(&input);

        assert_eq!(resp, Some(40));
    }

    #[test]
    fn test_day08_part2() {
        let input = parse_input(TEST_INPUT);

        let resp = part2(&input);

        assert_eq!(resp, Some(25272));
    }
}
