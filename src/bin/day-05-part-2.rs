#![allow(dead_code)]

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
    fn subtract(&self, rhs: &Self) -> Vec<Self>;
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
    fn subtract(&self, rhs: &Self) -> Vec<Self> {
        let Some(intersection) = self.intersect(rhs) else {
            // the sets don't overlap
            return vec![self.clone()];
        };

        if &intersection == self {
            // rhs fully covers self
            return Vec::new();
        }

        if rhs.start > self.start && rhs.end < self.end {
            // rhs is fully contained in self
            vec![self.start..rhs.start, rhs.end..self.end]
        } else if rhs.end > self.start && rhs.start <= self.start {
            // rhs overlaps self on the left side
            vec![rhs.end..self.end]
        } else if rhs.start < self.end && rhs.end >= self.end {
            // rhs overlaps self on the right side
            vec![self.start..rhs.start]
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
    use super::Set;

    #[test]
    fn intersect() {
        assert_eq!((0..10).intersect(&(2..12)), Some(2..10));
        assert_eq!((2..12).intersect(&(0..10)), Some(2..10));

        assert_eq!((10..20).intersect(&(0..10)), Some(10..10));
        assert_eq!((0..10).intersect(&(10..20)), Some(10..10));

        assert_eq!((0..1).intersect(&(10..11)), None);
        assert_eq!((10..11).intersect(&(0..1)), None);
    }

    #[test]
    fn subtract() {
        assert_eq!((1..6).subtract(&(10..16)), vec![1..6]);
        assert_eq!((1..6).subtract(&(0..3)), vec![3..6]);
        assert_eq!((1..6).subtract(&(4..7)), vec![1..4]);
        assert_eq!((1..6).subtract(&(3..5)), vec![1..3, 5..6]);
        assert_eq!((1..6).subtract(&(1..6)), vec![]);
        assert_eq!((1..6).subtract(&(1..1)), vec![1..6]);
        assert_eq!((1..6).subtract(&(1..2)), vec![2..6]);
        assert_eq!((1..6).subtract(&(5..6)), vec![1..5]);
        assert_eq!((1..6).subtract(&(6..6)), vec![1..6]);
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

    fn map(&self, range: &Range<u64>) -> Vec<Range<u64>> {
        // _ _ 3 4 5 6 _ _ |
        // 1 _ _ _ _ _ _ _ | no overlap
        // _ 2 3 _ _ _ _ _ | partial overlap
        // _ _ _ 4 5 _ _ _ | full overlap
        // _ _ _ _ _ 6 7 _ | partial overlap
        // _ _ _ _ _ _ _ 8 | no overlap
        // 1 2 3 4 5 6 7 8 | all of the above

        let mapped_range = self.src_range();

        let mut to_map = vec![range.clone()];
        let mut mapped = Vec::<Range<u64>>::new();

        loop {
            let Some(next) = to_map.pop() else {
                break;
            };

            if let Some(intersection) = mapped_range.intersect(&next) {
                let mapped_start = (intersection.start + self.dst_start) - self.src_start;
                let mapped_end = (intersection.end + self.dst_start) - self.src_start;

                // append the mapped range
                mapped.push(mapped_start..mapped_end);
                // add the unmapped parts back to the list
                to_map.extend(next.subtract(&intersection));
            }
        }

        eprintln!(
            "MapRange::map\n\
                \trange: {:?}\n\
                \tsrc: {:?}\n\
                \tdst: {:?}\n\
                \tmapped: {:?}",
            range,
            self.src_range(),
            self.dst_range(),
            mapped,
        );

        mapped
    }
}

#[cfg(test)]
mod map_range_test {
    use crate::{Map, MapRange};

    #[test]
    fn map_range() {
        let seeds = vec![79..(79 + 14), 55..(55 + 13)];
        let seed_to_soil = Map {
            name: "seed-to-soil".to_string(),
            ranges: vec![
                MapRange {
                    dst_start: 50,
                    src_start: 98,
                    len: 2,
                },
                MapRange {
                    dst_start: 52,
                    src_start: 50,
                    len: 48,
                },
            ],
        };
        let soil_to_fert = Map {
            name: "soil-to-fertilizer".to_string(),
            ranges: vec![
                MapRange {
                    dst_start: 0,
                    src_start: 15,
                    len: 37,
                },
                MapRange {
                    dst_start: 32,
                    src_start: 52,
                    len: 2,
                },
                MapRange {
                    dst_start: 39,
                    src_start: 0,
                    len: 15,
                },
            ],
        };

        let mut soils = seeds
            .iter()
            .map(|seed| seed_to_soil.map(seed))
            .flatten()
            .collect::<Vec<_>>();
        soils.sort_unstable_by_key(|r| (r.start, r.end));

        let mut ferts = soils
            .iter()
            .map(|soil| soil_to_fert.map(soil))
            .flatten()
            .collect::<Vec<_>>();
        ferts.sort_unstable_by_key(|r| (r.start, r.end));

        println!("seeds: {:?}", seeds);
        println!("soils: {:?}", soils);
        println!("ferts: {:?}", ferts);
    }
}

#[derive(Debug, Default)]
struct Map {
    name: String,
    ranges: Vec<MapRange>,
}

impl Map {
    fn map(&self, seeds: &Range<u64>) -> Vec<Range<u64>> {
        let mut mapped = vec![];

        for map in &self.ranges {
            mapped.extend(map.map(seeds));
        }

        mapped
    }
}

#[derive(Debug, Default)]
struct Almanac {
    seeds: Vec<Range<u64>>,
    maps: Vec<Map>,
}

fn main() {
    let input = advent_of_code_2023::load_input("day-05.txt");
    let start = std::time::Instant::now();

    let almanac = {
        let mut lines = input.lines();

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

            ranges.sort_unstable_by_key(|map_range| map_range.src_start);

            maps.push(Map {
                name: name.to_string(),
                ranges,
            });
        }

        Almanac { seeds, maps }
    };

    let mut buffer = almanac.seeds.clone();
    for map in &almanac.maps {
        let mut next = buffer
            .iter()
            .map(|range| map.map(range))
            .flatten()
            .collect::<Vec<_>>();

        next.sort_unstable_by_key(|r| (r.start, r.end));
        next.dedup_by_key(|r| (r.start, r.end));

        buffer = next;
    }

    let result = buffer.iter().map(|range| range.start).min().unwrap();

    let elapsed = start.elapsed().as_secs_f64() * 1e3;
    println!("{} ({:.3}ms)", result, elapsed);
}
