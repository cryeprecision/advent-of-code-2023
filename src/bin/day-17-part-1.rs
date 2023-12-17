use std::fmt::Write;

use smallvec::SmallVec;

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
    fn neighbors(&self, pos: usize) -> [Option<usize>; 4] {
        [
            Dir::Up.move_point(pos, self),
            Dir::Down.move_point(pos, self),
            Dir::Left.move_point(pos, self),
            Dir::Right.move_point(pos, self),
        ]
    }
    fn neighbors_in_queue(&self, queue: &[usize], vertex: usize) -> SmallVec<[usize; 4]> {
        self.neighbors(vertex)
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

#[derive(Debug, Clone, Copy)]
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
                    write!(f, " {:>3}", dist)?
                }
            }
            Ok(())
        };

        let write_prev_row =
            |f: &mut std::fmt::Formatter<'_>, row: &[Option<usize>]| -> std::fmt::Result {
                let mut iter = row
                    .iter()
                    .map(|idx| idx.map(|idx| (idx / self.graph.width, idx % self.graph.width)));
                if let Some(prev) = iter.next() {
                    match prev {
                        Some((row, col)) => write!(f, "[{:>2}, {:>2}]", row, col),
                        None => write!(f, "[__, __]"),
                    }?;
                    for prev in iter {
                        match prev {
                            Some((row, col)) => write!(f, " [{:>2}, {:>2}]", row, col),
                            None => write!(f, " [__, __]"),
                        }?;
                    }
                }
                Ok(())
            };

        let mut dist_rows = rows.clone();
        if let Some(row_idx) = dist_rows.next() {
            write_dist_row(f, self.graph.row_of(&self.dist, row_idx))?;
            for row_idx in dist_rows {
                f.write_char('\n')?;
                write_dist_row(f, self.graph.row_of(&self.dist, row_idx))?;
            }
        }

        f.write_str("\n\n")?;

        let mut prev_rows = rows.clone();
        if let Some(row_idx) = prev_rows.next() {
            write_prev_row(f, self.graph.row_of(&self.prev, row_idx))?;
            for row_idx in prev_rows {
                f.write_char('\n')?;
                write_prev_row(f, self.graph.row_of(&self.prev, row_idx))?;
            }
        }

        Ok(())
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
        for neighbor in graph.neighbors_in_queue(&queue, vertex) {
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

    println!("{:?}\n\n", graph);

    let dijkstra = dijkstra(&graph, 0);
    println!("{:?}", dijkstra);

    challenge.finish(0);
}
