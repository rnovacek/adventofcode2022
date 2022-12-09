use std::io::{Read, BufReader};

#[derive(Debug)]
enum Direction {
    LeftToRight,
    RightToLeft,
    TopToBottom,
    BottomToTop,
}

struct Forest {
    tree_heighs: Vec<u8>,
    tree_visibility: Vec<bool>,
    width: usize,
    height: usize,
}

impl Forest {
    fn new() -> Self {
        Self {
            tree_heighs: Vec::new(),
            tree_visibility: Vec::new(),
            width: 0,
            height: 0,
        }
    }

    fn add(&mut self, item: u8) {
        if self.height == 0 {
            self.height = 1;
        }
        self.tree_heighs.push(item);
        self.tree_visibility.push(false);
    }

    fn eol(&mut self) {
        if self.width == 0 {
            self.width = self.tree_heighs.len();
        }
        self.height += 1;
    }

    fn dir_next(&self, dir: &Direction, index: usize) -> Option<usize> {
        match dir {
            Direction::LeftToRight => {
                if index % self.width == self.width - 1 {
                    return None
                }
                return Some(index.wrapping_add(1))
            },
            Direction::RightToLeft => {
                if index % self.width == 0 {
                    return None
                }
                return Some(index.wrapping_sub(1))
            },
            Direction::TopToBottom => {
                if index >= (self.height - 1) * self.width {
                    return None
                }
                return Some(index.wrapping_add(self.width))
            }
            Direction::BottomToTop => {
                if index < self.width {
                    return None
                }
                return Some(index.wrapping_sub(self.width))
            }
        }
    }

    fn update_visibility(&mut self) {
        for i in 0..self.height {
            self.update_vector_visibility(&Direction::LeftToRight, i);
            self.update_vector_visibility(&Direction::RightToLeft, i);
        }

        for i in 0..self.width {
            self.update_vector_visibility(&Direction::TopToBottom, i);
            self.update_vector_visibility(&Direction::BottomToTop, i);
        }
    }

    fn update_vector_visibility(&mut self, dir: &Direction, index: usize) {
        let from: usize;
        from = match dir {
            Direction::LeftToRight => index * self.width,
            Direction::RightToLeft => (index + 1) * self.width - 1,
            Direction::TopToBottom => index,
            Direction::BottomToTop => self.width * (self.height - 1) + index,
        };

        let mut max: Option<u8> = None;
        let mut index = from;
        loop {
            let tree_height = self.tree_heighs[index];

            match max {
                Some(m) => {
                    if tree_height > m {
                        max = Some(tree_height);
                        self.tree_visibility[index] = true;
                    }
                },
                None => {
                    max = Some(tree_height);
                    self.tree_visibility[index] = true;
                }
            }
            match self.dir_next(dir, index) {
                Some(v) => {
                    index = v
                },
                None => break
            }
        }
    }

    fn visible_count(&self) -> usize {
        let mut count: usize = 0;
        for v in self.tree_visibility.iter() {
            if *v {
                count += 1
            }
        }
        return count;
    }

    fn best_scenic_score(&self) -> u32 {
        let mut best = 0u32;
        for index in 0..self.tree_heighs.len() {
            let score = self.scenic_score(index);
            if score > best {
                best = score;
            }
        }
        return best;
    }

    fn scenic_score(&self, index: usize) -> u32 {
        self.dir_scenic_score(&Direction::LeftToRight, index) *
        self.dir_scenic_score(&Direction::RightToLeft, index) *
        self.dir_scenic_score(&Direction::TopToBottom, index) *
        self.dir_scenic_score(&Direction::BottomToTop, index)
    }

    fn dir_scenic_score(&self, dir: &Direction, index: usize) -> u32 {
        let height = self.tree_heighs[index];
        let mut maybe_next_index = self.dir_next(dir, index);
        let mut count = 0;
        loop {
            let next_index;
            match maybe_next_index {
                Some(i) => next_index = i,
                None => break,
            }
            let h = self.tree_heighs[next_index];

            count += 1;
            if h >= height {
                break;
            }

            maybe_next_index = self.dir_next(dir, next_index);
        }
        return count;

    }
}

pub fn run<R: Read>(input: BufReader<R>) -> Result<(String, String), String> {
    let mut forest = Forest::new();
    for item in input.bytes() {
        match item {
            Ok(b'\n') => forest.eol(),
            Ok(num) if num >= b'0' && num <= b'9' => forest.add(num - b'0'),
            Ok(non_num) => return Err(format!("Invalid input char {}", non_num)),
            Err(e) => return Err(format!("Unable to read input: {}", e.to_string())),
        }
    }
    forest.update_visibility();

    Ok((
        format!("{}", forest.visible_count()),
        format!("{}", forest.best_scenic_score()),
    ))
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::test_util::str_to_buf_reader;

    #[test]
    fn test_d08_01() {
        let input = String::from("30373
25512
65332
33549
35390");
        let result = run(str_to_buf_reader(&input)).expect("Run failed");
        assert_eq!(
            result.0,
            "21",
        );
        assert_eq!(
            result.1,
            "8",
        );
    }

    #[test]
    fn test_d08_final() {
        let f = std::fs::File::open("src/d08/input.txt").expect("No src/d08/input.txt file");
        let result = run(BufReader::new(f)).expect("Run failed");
        assert_eq!(
            result.0,
            "1787",
        );
        assert_eq!(
            result.1,
            "440640",
        );
    }
}
