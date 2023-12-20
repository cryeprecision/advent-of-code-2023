#![feature(iter_map_windows)]

use std::{collections::VecDeque, fmt::Write};

use smallvec::SmallVec;

#[derive(Clone)]
struct Graph {
    data: Vec<u8>,
    width: usize,
}

impl Graph {
    fn row(&self, row_idx: usize) -> &[u8] {
        &self.data[(row_idx * self.width)..((row_idx + 1) * self.width)]
    }
    fn row_of<'a, T>(&self, other: &'a [T], row_idx: usize) -> &'a [T] {
        &other[(row_idx * self.width)..((row_idx + 1) * self.width)]
    }
    fn height(&self) -> usize {
        self.data.len() / self.width
    }

    /// Returns `[Up, Down, Left, Right]`
    fn neighbors(
        &self,
        pos: usize,
        streak: Option<Dir>,
        last_dir: Option<Dir>,
    ) -> [Option<usize>; 4] {
        let mut neighbors = [
            Dir::Up.move_point(pos, self),
            Dir::Down.move_point(pos, self),
            Dir::Left.move_point(pos, self),
            Dir::Right.move_point(pos, self),
        ];
        match streak {
            None => (),
            Some(Dir::Up) => neighbors[0] = None,
            Some(Dir::Down) => neighbors[1] = None,
            Some(Dir::Left) => neighbors[2] = None,
            Some(Dir::Right) => neighbors[3] = None,
        }
        match last_dir {
            None => (),
            Some(Dir::Up) => neighbors[1] = None,
            Some(Dir::Down) => neighbors[0] = None,
            Some(Dir::Left) => neighbors[3] = None,
            Some(Dir::Right) => neighbors[2] = None,
        }
        neighbors
    }

    /// Return an arrow character pointing from `from` to `to`
    fn dir_to(&self, from: usize, to: usize) -> Option<Dir> {
        if from + 1 == to {
            Some(Dir::Right)
        } else if from == to + 1 {
            Some(Dir::Left)
        } else if from + self.width == to {
            Some(Dir::Down)
        } else if from == to + self.width {
            Some(Dir::Up)
        } else {
            None
        }
    }

    fn has_streak(&self, vertex: usize, prev: &[Option<usize>]) -> Option<Dir> {
        let mut curr_prev = prev[vertex]?;
        let first_dir = self.dir_to(curr_prev, vertex).unwrap();

        for _ in 0..2 {
            let tmp_prev = prev[curr_prev]?;
            if self.dir_to(tmp_prev, curr_prev).unwrap() != first_dir {
                return None;
            }
            curr_prev = prev[curr_prev]?;
        }

        Some(first_dir)
    }

    fn neighbors_in_queue(
        &self,
        vertex: usize,
        queue: &[usize],
        prev: &[Option<usize>],
    ) -> SmallVec<[usize; 4]> {
        let streak = self.has_streak(vertex, prev);
        let last_dir = prev[vertex].and_then(|prev| self.dir_to(prev, vertex));
        self.neighbors(vertex, streak, last_dir)
            .into_iter()
            .filter_map(|neighbor| match neighbor {
                None => None,
                Some(vertex) if queue.contains(&vertex) => Some(vertex),
                Some(_) => None,
            })
            .collect()
    }

    fn number_at(&self, pos: usize) -> u8 {
        // The graph consists only of ASCII numbers
        self.data[pos] - b'0'
    }
}

impl std::fmt::Debug for Graph {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut rows = 0..self.height();
        if let Some(row_idx) = rows.next() {
            f.write_str(std::str::from_utf8(self.row(row_idx)).unwrap())?;
            for row_idx in rows {
                f.write_char('\n')?;
                f.write_str(std::str::from_utf8(self.row(row_idx)).unwrap())?;
            }
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Dir {
    Up,
    Down,
    Left,
    Right,
}

impl Dir {
    fn move_point(self, point: usize, graph: &Graph) -> Option<usize> {
        match self {
            Dir::Up if point >= graph.width => Some(point - graph.width),
            Dir::Down if point < graph.data.len() - graph.width => Some(point + graph.width),
            Dir::Left if point % graph.width != 0 => Some(point - 1),
            Dir::Right if point % graph.width != graph.width - 1 => Some(point + 1),
            _ => None,
        }
    }
}

struct Dijkstra<'a> {
    graph: &'a Graph,
    dist: Vec<u32>,
    prev: Vec<Option<usize>>,
}

impl std::fmt::Debug for Dijkstra<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let rows = 0..self.graph.height();

        let write_dist_row = |f: &mut std::fmt::Formatter<'_>, row: &[u32]| -> std::fmt::Result {
            let mut iter = row.iter();
            if let Some(dist) = iter.next() {
                write!(f, "{:>3}", dist)?;
                for dist in iter {
                    f.write_char(' ')?;
                    write!(f, "{:>3}", dist)?
                }
            }
            Ok(())
        };

        f.write_str("[i] Distances:\n")?;

        let mut dist_rows = rows.clone();
        if let Some(row_idx) = dist_rows.next() {
            write_dist_row(f, self.graph.row_of(&self.dist, row_idx))?;
            for row_idx in dist_rows {
                f.write_char('\n')?;
                write_dist_row(f, self.graph.row_of(&self.dist, row_idx))?;
            }
        }

        f.write_str("\n\n[i] Previous Nodes:\n")?;

        for row_idx in 0..self.graph.height() {
            if row_idx != 0 {
                f.write_char('\n')?;
            }
            for col_idx in 0..self.graph.width {
                let idx = row_idx * self.graph.width + col_idx;
                let prev_char = match self.prev[row_idx * self.graph.width + col_idx] {
                    None => '_',
                    Some(prev) if prev + 1 == idx => '←',
                    Some(prev) if prev == idx + 1 => '→',
                    Some(prev) if prev + self.graph.width == idx => '↑',
                    Some(prev) if prev == idx + self.graph.width => '↓',
                    _ => panic!("invalid neighbour"),
                };
                if col_idx != 0 {
                    f.write_char(' ')?;
                }
                f.write_char(prev_char)?;
            }
        }

        Ok(())
    }
}

impl Dijkstra<'_> {
    fn debug_shortest_path_to(&self, vertex: usize) -> Graph {
        let mut data = vec![b'_'; self.graph.data.len()];
        self.shortest_path_to(vertex)
            .iter()
            .for_each(|&idx| data[idx] = b'X');
        Graph {
            data,
            width: self.graph.width,
        }
    }

    fn shortest_path_to(&self, vertex: usize) -> Vec<usize> {
        let Some(mut prev) = self.prev[vertex] else {
            return Vec::new();
        };
        let mut path = vec![vertex, prev];
        while let Some(next_prev) = self.prev[prev] {
            path.push(next_prev);
            prev = next_prev;
        }
        path.reverse();
        path
    }
}

fn dijkstra(graph: &Graph, start: usize) -> Dijkstra {
    // Run Dijkstra's algorithm on the graph G = (V, E) but since we have vertex weights and need
    // edge weights, we need to define a custom weight function for edges as follows
    // => w'(u, v) = w(v) where w: V -> ℕ and w': E -> ℕ

    fn pop_min_dist_vertex(queue: &mut Vec<usize>, dist: &[u32]) -> Option<(usize, u32)> {
        let (queue_idx, vertex, min_dist) = queue
            .iter()
            .enumerate()
            .map(|(queue_idx, &vertex)| (queue_idx, vertex, dist[vertex]))
            .min_by_key(|&(_, _, dist)| dist)?;

        // remove the vertex with minimal weight from the queue
        queue.remove(queue_idx);

        // return the removed vertex with its weight
        Some((vertex, min_dist))
    }

    // current distances from the start vertex
    let mut dist: Vec<u32> = vec![u32::MAX; graph.data.len()];

    // previous-hop node on the shortest path from start to the vertex
    let mut prev: Vec<Option<usize>> = vec![None; graph.data.len()];

    // vertices left to process
    let mut queue: Vec<usize> = (0..graph.data.len()).collect();

    // the start node has a distance to itself because of the custom weight function
    dist[start] = u32::from(graph.number_at(start));

    while !queue.is_empty() {
        // u ← vertex in Q with min dist[u]; remove u from Q
        let (vertex, vertex_dist) = pop_min_dist_vertex(&mut queue, &dist).unwrap();

        // for each neighbor v of u still in Q:
        for neighbor in graph.neighbors_in_queue(vertex, &queue, &prev) {
            // alt ← dist[u] + Graph.Edges(u, v)
            let alternative = vertex_dist + u32::from(graph.number_at(neighbor));

            // if alt < dist[v]:
            if alternative < dist[neighbor] {
                // dist[v] ← alt; prev[v] ← u
                dist[neighbor] = alternative;
                prev[neighbor] = Some(vertex);
            }
        }
    }

    Dijkstra { graph, dist, prev }
}

fn main() {
    let mut challenge = advent_of_code_2023::Challenge::start(17, 1);

    let graph = {
        let width = challenge.input_lines().next().unwrap().len();
        let mut data = Vec::new();
        challenge
            .input_lines()
            .for_each(|line| data.extend_from_slice(line.as_bytes()));

        Graph { data, width }
    };

    challenge.finish_parsing();

    let dijkstraa = dijkstra(&graph, 0);

    println!("{:?}\n\n", graph);
    println!("{:?}\n\n", dijkstraa);
    println!(
        "{:?}\n\n",
        dijkstraa.debug_shortest_path_to(graph.data.len() - 1)
    );
    println!(
        "{:?}",
        dijkstraa
            .shortest_path_to(graph.data.len() - 1)
            .into_iter()
            .map(|idx| (idx / graph.width, idx % graph.width))
            .collect::<Vec<_>>()
    );

    let solution = dijkstraa
        .shortest_path_to(graph.data.len() - 1)
        .iter()
        .map(|&vertex| graph.number_at(vertex) as u64)
        .sum::<u64>();

    challenge.finish(solution);
}
