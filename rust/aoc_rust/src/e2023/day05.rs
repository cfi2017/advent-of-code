use std::collections::HashMap;
use std::iter::Map;
use std::ops::Range;
use std::str::Split;
use itertools::Itertools;
use crate::aoc_boilerplate;

pub struct ArrayButWithHoles {
    ranges: HashMap<Range<i64>, Range<i64>>
}

impl From<HashMap<Range<i64>, Range<i64>>> for ArrayButWithHoles {
    fn from(value: HashMap<Range<i64>, Range<i64>>) -> Self {
        ArrayButWithHoles {
            ranges: value,
        }
    }
}

impl ArrayButWithHoles {
    pub fn get(&self, index: i64) -> i64 {
        for (range, value) in self.ranges.iter() {
            if range.contains(&index) {
                return value.start + index - range.start;
            }
        }
        index
    }

    // maps input ranges to output ranges
    // len(output) >= len(input)
    pub fn get_ranges<'a>(&self, mut ranges: &'a mut Vec<Range<i64>>, mut next: &'a mut Vec<Range<i64>>) {
        let mut result = Vec::with_capacity(ranges.len() * 2);

        for (source, dest) in self.ranges.iter() {
            // 0 [ 1 2 3 4 ] 5 6 [ 7 8 9 ]
            // 0 [ 2 3 4 5 ] 5 6 [ 2 3 4 ]
            while let Some(range) = ranges.pop() {
                let before = range.start..range.end.min(source.start);
                let contained = range.start.max(source.start)..range.end.min(source.end);
                let after = range.start.max(source.end)..range.end;
                if !before.is_empty() {
                    next.push(before);
                }
                if !contained.is_empty() {
                    result.push((contained.start - source.start + dest.start)..(contained.end - source.start + dest.start));
                }
                if !after.is_empty() {
                    next.push(after);
                }
            }
            std::mem::swap(&mut ranges, &mut next);

        }
        // add result to ranges
        ranges.append(&mut result);
    }
}

aoc_boilerplate!(2023, 5, sanitize_input_a, sanitize_input_b, solve_a, solve_b);

pub fn sanitize_input_a(input: &str) -> (Vec<i64>, Vec<ArrayButWithHoles>) {
    let mut split = input.split("\n\n");
    let seeds = parse_seeds(&mut split).collect::<Vec<i64>>();
    let maps = get_holy_arrays(split);
    (seeds, maps)
}

fn parse_seeds<'a>(split: &mut impl Iterator<Item=&'a str>) -> impl Iterator<Item=i64> + 'a {
    split.next().unwrap().split(": ").nth(1).unwrap().split(' ').map(|x| x.parse::<i64>().unwrap())
}

pub fn sanitize_input_b(input: &str) -> (Vec<Range<i64>>, Vec<ArrayButWithHoles>) {
    let mut split = input.split("\n\n");
    let seeds = parse_seeds(&mut split)
        .chunks(2)
        .into_iter()
        .map(|mut x| (x.next().unwrap(), x.next().unwrap()))
        .map(|(start, len)| start..start + len)
        .collect::<Vec<Range<i64>>>();

    let maps = get_holy_arrays(split);
    (seeds, maps)
}

fn get_holy_arrays(split: impl Iterator<Item=&str>) -> Vec<ArrayButWithHoles> {
    split.map(|weirdmap| {
        let mut lines = weirdmap.lines();
        let _ = lines.next();
        let ranges: HashMap<Range<i64>, Range<i64>> = lines.map(|line| line.split(' ').map(|x| x.parse::<i64>().unwrap()).collect_tuple::<(i64, i64, i64)>().unwrap())
            .map(|(dest, source, len)| (source..source + len, dest..dest + len))
            .collect();
        ArrayButWithHoles::from(ranges)
    }).collect()
}

pub fn solve_a((seeds, maps): (Vec<i64>, Vec<ArrayButWithHoles>)) -> i64 {
    let mut result = seeds.clone();
    for map in maps {
        result = result.iter().map(|x| map.get(*x)).collect();
    }
    *result.iter().min().unwrap()
}

pub fn solve_b((seeds, maps): (Vec<Range<i64>>, Vec<ArrayButWithHoles>)) -> i64 {
    let mut result = seeds.clone();
    let mut next = Vec::with_capacity(50);
    for map in maps {
        map.get_ranges(&mut result, &mut next);
        if !next.is_empty() {
            std::mem::swap(&mut result, &mut next);
        }
    }
    result.iter().map(|r| r.start).min().unwrap()
}

#[cfg(test)]
mod tests {
    use crate::add_test;
    use super::*;

    #[test]
    fn test_weird_map() {
        let map = ArrayButWithHoles::from(vec![
            (1..5, 2..6),
            (7..10, 2..5),
        ].into_iter().collect::<HashMap<_, _>>());

        assert_eq!(map.get(0), 0);
        assert_eq!(map.get(1), 2);
        assert_eq!(map.get(2), 3);
        assert_eq!(map.get(3), 4);
        assert_eq!(map.get(4), 5);
        assert_eq!(map.get(5), 5);
        assert_eq!(map.get(6), 6);
        assert_eq!(map.get(7), 2);
        assert_eq!(map.get(8), 3);
        assert_eq!(map.get(9), 4);

        let mut ranges = vec![1..5];
        let mut next = Vec::with_capacity(50);
        map.get_ranges(&mut ranges, &mut next);
        assert_eq!(ranges, vec![2..6]);
    }

    add_test!(test_solve_a_example, sanitize_input_a, solve_a, r#"seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4"#, 35);

    add_test!(test_solve_b_example, sanitize_input_b, solve_b, r#"seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4"#, 46);
}
