#![allow(dead_code)]

use smallvec::{smallvec, SmallVec};
use std::ops::Range;

/// - Any source numbers that aren't mapped correspond to the same destination number.
#[derive(Debug)]
struct MapRange {
    src_start: u64,
    dst_start: u64,
    len: u64,
}

trait Set: Sized {
    /// Set intersection of self with rhs, this operation is commutative.
    fn intersect(&self, rhs: &Self) -> Option<Self>;

    /// Subtract rhs from self.
    fn subtract(&self, rhs: &Self) -> SmallVec<[Self; 2]>;
}

impl Set for Range<u64> {
    fn intersect(&self, other: &Self) -> Option<Self> {
        let start = self.start.max(other.start);
        let end = self.end.min(other.end);

        if start >= end {
            None
        } else {
            Some(start..end)
        }
    }
    fn subtract(&self, rhs: &Self) -> SmallVec<[Self; 2]> {
        let Some(intersection) = self.intersect(rhs) else {
            // the sets don't overlap
            return smallvec![self.clone()];
        };

        if &intersection == self {
            // rhs fully covers self
            return smallvec![];
        }

        if rhs.start > self.start && rhs.end < self.end {
            // rhs is fully contained in self
            smallvec![self.start..rhs.start, rhs.end..self.end]
        } else if rhs.end > self.start && rhs.start <= self.start {
            // rhs overlaps self on the left side
            smallvec![rhs.end..self.end]
        } else if rhs.start < self.end && rhs.end >= self.end {
            // rhs overlaps self on the right side
            smallvec![self.start..rhs.start]
        } else {
            unreachable!(
                "all cases should have been covered by the code above\n\
                    \tself: {:?}\n\
                    \trhs: {:?}\n\
                    \tintersection: {:?}",
                self, rhs, intersection
            );
        }
    }
}

#[cfg(test)]
mod set_test {
    use super::{Range, Set};
    use smallvec::{smallvec, SmallVec};

    #[test]
    fn intersect() {
        macro_rules! test_eq {
            ($lhs:expr, $rhs:expr => []) => {
                assert_eq!(($lhs).intersect(&($rhs)), None);
                assert_eq!(($rhs).intersect(&($lhs)), None);
            };
            ($lhs:expr, $rhs:expr => [$result:expr]) => {
                assert_eq!(($lhs).intersect(&($rhs)), Some($result));
                assert_eq!(($rhs).intersect(&($lhs)), Some($result));
            };
        }

        test_eq!(0..10, 2..12 => [2..10]);
        test_eq!(10..20, 0..10 => []);
        test_eq!(0..1, 10..11 => []);
    }

    #[test]
    fn subtract() {
        macro_rules! test_eq {
            ($base:expr, $sub:expr => [$($equals:expr),*]) => {{
                let expected: SmallVec::<[Range<u64>; 2]> = smallvec![$($equals),*];
                assert_eq!(($base).subtract(&($sub)), expected);
            }};
        }

        test_eq!(1..6, 10..16 => [1..6]);
        test_eq!(1..6, 0..3 => [3..6]);
        test_eq!(1..6, 4..7 => [1..4]);
        test_eq!(1..6, 3..5 => [1..3, 5..6]);
        test_eq!(1..6, 1..6 => []);
        test_eq!(1..6, 1..1 => [1..6]);
        test_eq!(1..6, 1..2 => [2..6]);
        test_eq!(1..6, 5..6 => [1..5]);
        test_eq!(1..6, 6..6 => [1..6]);
    }
}

impl MapRange {
    fn src_end(&self) -> u64 {
        self.src_start + self.len
    }
    fn dst_end(&self) -> u64 {
        self.dst_start + self.len
    }
    fn src_range(&self) -> Range<u64> {
        self.src_start..self.src_end()
    }
    fn dst_range(&self) -> Range<u64> {
        self.dst_start..self.dst_end()
    }

    /// Returns: `(MappedRange, UnmappedRanges)`
    ///
    /// If there is an intersection with the range this maps, that part will be mapped
    /// and the unmapped part(s) will be put into the second tuple element.
    fn map(&self, range: &Range<u64>) -> (Option<Range<u64>>, SmallVec<[Range<u64>; 2]>) {
        if let Some(intersection) = self.src_range().intersect(range) {
            let mapped_start = (intersection.start + self.dst_start) - self.src_start;
            let mapped_end = (intersection.end + self.dst_start) - self.src_start;
            let unmapped = range.subtract(&intersection);
            (Some(mapped_start..mapped_end), unmapped)
        } else {
            (None, smallvec![range.clone()])
        }
    }
}

#[derive(Debug, Default)]
struct Map {
    name: String,
    mappings: Vec<MapRange>,
}

impl Map {
    /// Tries to map as much as possible from the seeds
    ///
    /// Returns: `(MappedSeeds, UnmappedSeeds)`
    fn map(&self, seeds: &Range<u64>) -> Vec<Range<u64>> {
        let mut mapped = vec![];
        let mut unmapped = vec![seeds.clone()];
        let mut unmapped_temp = vec![];

        for map in &self.mappings {
            for seed in unmapped.iter() {
                let (mapped_range, unmapped_ranges) = map.map(seed);

                if let Some(mapped_range) = mapped_range {
                    mapped.push(mapped_range);
                }
                unmapped_temp.extend(unmapped_ranges);
            }
            unmapped.clear();
            std::mem::swap(&mut unmapped, &mut unmapped_temp);
        }

        mapped.extend(unmapped);
        mapped
    }
}

#[derive(Debug, Default)]
struct Almanac {
    seeds: Vec<Range<u64>>,
    maps: Vec<Map>,
}

fn main() {
    let challenge = advent_of_code_2023::Challenge::start(5, 2);

    let almanac = {
        let mut lines = challenge.input_lines();

        let seeds_line = lines.next().unwrap();
        let (_, seeds_list) = seeds_line.split_once("seeds: ").unwrap();
        let mut seed_nums = seeds_list.split_whitespace();

        let mut seeds = Vec::new();
        loop {
            let Some(start) = seed_nums.next() else {
                break;
            };
            let start = start.parse::<u64>().unwrap();
            let len = seed_nums.next().unwrap().parse::<u64>().unwrap();

            seeds.push(start..(start + len));
        }

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

            ranges.sort_unstable_by_key(|r| (r.src_start, r.src_end()));

            maps.push(Map {
                name: name.to_string(),
                mappings: ranges,
            });
        }

        Almanac { seeds, maps }
    };

    let result = {
        let mut ranges = almanac.seeds.clone();
        for map in &almanac.maps {
            ranges = ranges
                .iter()
                .flat_map(|range| map.map(range))
                .collect::<Vec<_>>();
        }
        ranges.iter().map(|r| r.start).min().unwrap()
    };

    challenge.finish(result);
}
