use std::{io::{Read, BufReader}, collections::{BinaryHeap, HashMap}, cmp::Ordering};

#[derive(Debug)]
struct Node {
    index: usize,
    distance: usize,
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.distance.cmp(&other.distance).reverse()
    }
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Node {
    fn eq(&self, other: &Self) -> bool {
        self.distance == other.distance && self.index == other.index
    }
}

impl Eq for Node {}

#[derive(Debug)]
struct Matrix {
    items: Vec<u8>,
    start: usize,
    end: usize,
    width: usize,
}

impl Matrix {
    fn new() -> Self {
        Self {
            items: Vec::new(),
            start: 0,
            end: 0,
            width: 0,
        }
    }

    fn add_char(self: &mut Self, ch: u8) {
        match ch {
            b'\n' => {
                if self.width == 0 {
                    self.width = self.items.len();
                }
            },
            b'S' => {
                self.start = self.items.len();
                self.items.push(0u8);
            },
            b'E' => {
                self.end = self.items.len();
                self.items.push(25u8);
           },
            n => {
                self.items.push(n - b'a');
            },
        }
    }
}

fn dijkstra(matrix: &Matrix, previous: &mut HashMap<usize, usize>, first_level_cost: usize) -> bool {
    let mut neighbours: Vec<usize> = Vec::new();
    neighbours.reserve(4);
    let mut heap: BinaryHeap<Node> = BinaryHeap::new();
    heap.push(Node { index: matrix.start, distance: 0 });

    loop {
        let item = heap.pop();
        match item {
            Some(node) => {
                let height = matrix.items[node.index];

                if node.index >= matrix.width {
                    neighbours.push(node.index - matrix.width);
                }
                if node.index <= matrix.items.len() - matrix.width {
                    neighbours.push(node.index + matrix.width);
                }
                if node.index % matrix.width > 0 {
                    neighbours.push(node.index - 1);
                }
                if (node.index % matrix.width) < matrix.width {
                    neighbours.push(node.index + 1);
                }

                for neighbour_index in neighbours.drain(..) {
                    let neighbour_height = match matrix.items.get(neighbour_index) {
                        Some(n) => n,
                        None => {
                            continue;
                        },
                    };

                    if *neighbour_height > height + 1 {
                        continue;
                    }

                    // We've been here, go to next neighbour
                    match previous.get(&neighbour_index) {
                        Some(_) => {
                            continue;
                        },
                        None => {}
                    }

                    previous.insert(neighbour_index, node.index);
                    if neighbour_index == matrix.end {
                        return true;
                    }

                    let mut new_distance = node.distance + 1;
                    if height == 0 && *neighbour_height == 0 {
                        new_distance = node.distance + first_level_cost;
                    }

                    heap.push(Node { index: neighbour_index, distance: new_distance });
                }
            },
            None => return false,
        }
    }
}

fn _draw_result(matrix: &Matrix, previous: &HashMap<usize, usize>) {
    for (index, height) in matrix.items.iter().enumerate() {
        match previous.get(&index) {
            Some(_) => print!("{}", (height + b'A') as char),
            None => print!("{}", (height + b'a') as char),
        }
        if index % matrix.width == matrix.width - 1 {
            println!();
        }
    }
}

fn backtrack<F>(end: usize, previous: &HashMap<usize, usize>, is_finish: F) -> Result<usize, String> where F: Fn(usize) -> bool {
    let mut current = end;
    let mut count = 0;
    while !is_finish(current) {
        count += 1;
        current = match previous.get(&current) {
            Some(c) => *c,
            None => return Err(format!("Item with index {} not found in previous", current)),
        }
    }
    Ok(count)
}

pub fn run<R: Read>(input: BufReader<R>) -> Result<(String, String), String> {

    let mut matrix = Matrix::new();

    for item in input.bytes() {
        match item {
            Ok(ch) => matrix.add_char(ch),
            Err(e) => return Err(format!("Unable to read: {}", e.to_string())),
        }
    }

    let mut previous: HashMap<usize, usize> = HashMap::new();
    let result1 = dijkstra(&matrix, &mut previous, 1);

    // _draw_result(&matrix, &previous);
    // println!();

    if !result1 {
        return Err(String::from("No solution found for part 1"));
    }

    let count1 = match backtrack(matrix.end, &previous, move |x| x == matrix.start) {
        Ok(c) => c,
        Err(e) => return Err(e),
    };

    // PART TWO
    previous.drain();

    let result2 = dijkstra(&matrix, &mut previous, 0);

    // _draw_result(&matrix, &previous);

    if !result2 {
        return Err(String::from("No solution found for part 2"));
    }

    let count2 = match backtrack(matrix.end, &previous, |x| matrix.items[x] == 0) {
        Ok(c) => c,
        Err(e) => return Err(e),
    };

    Ok((
        format!("{}", count1),
        format!("{}", count2),
    ))
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::test_util::str_to_buf_reader;

    #[test]
    fn test_d12_01() {
        let input = String::from("Sabqponm
abcryxxl
accszExk
acctuvwj
abdefghi");
        let result = run(str_to_buf_reader(&input)).expect("Run failed");
        assert_eq!(
            result.0,
            "31",
        );
        assert_eq!(
            result.1,
            "29",
        );
    }

    #[test]
    fn test_d12_final() {
        let f = std::fs::File::open("src/d12/input.txt").expect("No src/12/input.txt file");
        let result = run(BufReader::new(f)).expect("Run failed");
        assert_eq!(
            result.0,
            "449",
        );
        assert_eq!(
            result.1,
            "443",
        );
    }
}