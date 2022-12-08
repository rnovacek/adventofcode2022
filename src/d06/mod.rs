use std::{io::{BufReader, Read}, collections::{HashSet, VecDeque}};

struct LimitedDequeue {
    q: VecDeque<char>,
    size: usize,
    unique: HashSet<char>,

}

impl LimitedDequeue {
    fn new(size: usize) -> Self {
        return Self {
            q: VecDeque::with_capacity(size),
            size,
            unique: HashSet::with_capacity(size),
        }
    }

    fn push(&mut self, el: char) {
        if self.q.len() == self.size {
            self.q.pop_front();
        }
        self.q.push_back(el);
    }

    fn all_unique(&mut self) -> bool {
        self.unique.drain();
        for ch in &self.q {
            self.unique.insert(*ch);
        }
        return self.unique.len() == self.size;
    }
}

pub fn run<R: Read>(mut input: BufReader<R>) -> Result<(String, String), String> {
    let mut buffer = [0]; // read one character at a time

    let mut first_last_chars: LimitedDequeue = LimitedDequeue::new(4);
    let mut second_last_chars: LimitedDequeue = LimitedDequeue::new(14);
    let mut first_end_index: Option<usize> = None;
    let mut second_end_index: Option<usize> = None;
    let mut index: usize = 1;
    loop {
        match input.read(&mut buffer) {
            Ok(0) => break,
            Ok(1) => {

            },
            Ok(x) => return Err(format!("Invalid read returning {} chars", x)),
            Err(e) => return Err(format!("Unable to read from file: {}", e))
        }
        let c = buffer[0] as char;

        first_last_chars.push(c);
        if first_end_index == None && first_last_chars.all_unique() {
            first_end_index = Some(index);
        }

        second_last_chars.push(c);
        if second_end_index == None && second_last_chars.all_unique() {
            second_end_index = Some(index);
        }


        index += 1;
    }

    return match (first_end_index, second_end_index) {
        (Some(first), Some(second)) => return Ok((first.to_string(), second.to_string())),
        (None, None) => Err(String::from("Both parts have no solution")),
        (None, _) => Err(String::from("First part has no solution")),
        (_, None) => Err(String::from("Second part has no solution")),
    };
}

#[cfg(test)]
mod test {
    use super::*;
    use std::fs::File;
    use crate::test_util::str_to_buf_reader;

    #[test]
    fn test_d06_01() {
        assert_eq!(
            run(str_to_buf_reader(&String::from("bvwbjplbgvbhsrlpgdmjqwftvncz"))).expect("Error").0,
            "5"
        );
    }

    #[test]
    fn test_d06_02() {
        assert_eq!(
            run(str_to_buf_reader(&String::from("nppdvjthqldpwncqszvftbrmjlhg"))).expect("Error").0,
            "6"
        );
    }

    #[test]
    fn test_d06_03() {
        assert_eq!(
            run(str_to_buf_reader(&String::from("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg"))).expect("Error").0,
            "10"
        );
    }

    #[test]
    fn test_d06_04() {
        assert_eq!(
            run(str_to_buf_reader(&String::from("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw"))).expect("Error").0,
            "11"
        );
    }

    #[test]
    fn test_d06_05() {
        assert_eq!(
            run(str_to_buf_reader(&String::from("mjqjpqmgbljsphdztnvjfqwrcgsmlb"))).expect("Error").1,
            "19"
        );
    }

    #[test]
    fn test_d06_06() {
        assert_eq!(
            run(str_to_buf_reader(&String::from("bvwbjplbgvbhsrlpgdmjqwftvncz"))).expect("Error").1,
            "23"
        );
    }


    #[test]
    fn test_d06_07() {
        assert_eq!(
            run(str_to_buf_reader(&String::from("nppdvjthqldpwncqszvftbrmjlhg"))).expect("Error").1,
            "23"
        );
    }

    #[test]
    fn test_d06_08() {
        assert_eq!(
            run(str_to_buf_reader(&String::from("nppdvjthqldpwncqszvftbrmjlhg"))).expect("Error").1,
            "23"
        );
    }

    #[test]
    fn test_d06_09() {
        assert_eq!(
            run(str_to_buf_reader(&String::from("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg"))).expect("Error").1,
            "29"
        );
    }


    #[test]
    fn test_d06_10() {
        assert_eq!(
            run(str_to_buf_reader(&String::from("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw"))).expect("Error").1,
            "26"
        );
    }

    #[test]
    fn test_d06_final() {
        let f = File::open("src/d06/input.txt").expect("No src/d06/input.txt file");
        let result = run(BufReader::new(f)).expect("Run failed");
        assert_eq!(
            result.0,
            "1707"
        );
        assert_eq!(
            result.1,
            "3697"
        );
    }
}
