#![allow(dead_code)]

use std::ops::Range;

/// - Any source numbers that aren't mapped correspond to the same destination number.
#[derive(Debug)]
struct MapRange {
    src_start: u64,
    dst_start: u64,
    len: u64,
}

impl MapRange {
    fn src_range(&self) -> Range<u64> {
        self.src_start..(self.src_start + self.len)
    }
    fn dst_range(&self) -> Range<u64> {
        self.dst_start..(self.dst_start + self.len)
    }
    fn map(&self, num: u64) -> Option<u64> {
        if self.src_range().contains(&num) {
            Some(self.dst_start + (num - self.src_start))
        } else {
            None
        }
    }
}

#[derive(Debug, Default)]
struct Map {
    name: String,
    ranges: Vec<MapRange>,
}

impl Map {
    fn map(&self, num: u64) -> u64 {
        self.ranges
            .iter()
            .filter_map(|range| range.map(num))
            .next()
            .unwrap_or(num)
    }
}

#[derive(Debug, Default)]
struct Almanac {
    seeds: Vec<u64>,
    maps: Vec<Map>,
}

fn main() {
    let challenge = advent_of_code_2023::Challenge::start(5, 1);

    let almanac = {
        let mut lines = challenge.input_lines();

        let seeds_line = lines.next().unwrap();
        let (_, seeds_list) = seeds_line.split_once("seeds: ").unwrap();
        let seeds = seeds_list
            .split_whitespace()
            .map(|num| num.parse::<u64>().unwrap())
            .collect::<Vec<_>>();

        // skip empty line
        let _ = lines.next().unwrap();

        let mut maps = Vec::new();
        loop {
            // check for start of a new map
            let Some(header) = lines.next() else {
                break;
            };
            let (name, _) = header.split_once(' ').unwrap();

            let mut ranges = Vec::new();

            // parse the ranges
            for map_line in &mut lines {
                // check for end of ranges
                if map_line.is_empty() {
                    break;
                }

                // parse the map
                let (dst, rem) = map_line.split_once(' ').unwrap();
                let (src, len) = rem.split_once(' ').unwrap();

                ranges.push(MapRange {
                    src_start: src.parse().unwrap(),
                    dst_start: dst.parse().unwrap(),
                    len: len.parse().unwrap(),
                });
            }

            maps.push(Map {
                name: name.to_string(),
                ranges,
            });
        }

        Almanac { seeds, maps }
    };

    let solution = almanac
        .seeds
        .iter()
        .map(|seed| almanac.maps.iter().fold(*seed, |acc, next| next.map(acc)))
        .min()
        .unwrap();

    challenge.finish(solution);
}
