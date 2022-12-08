use std::io::{ BufReader, BufRead, Read };

pub fn run<R: Read>(input: BufReader<R>) -> Result<(String, String), String> {
    let mut current_sum = 0;
    let mut total_max: i32 = 0;
    let mut top_three = Vec::new();
    for line in input.lines() {
        match line {
            Ok(text) => {
                match text.len() {
                    0 => {
                        if current_sum > total_max {
                            total_max = current_sum;
                        }
                        top_three.push(current_sum);
                        top_three.sort_unstable_by(|a, b| b.partial_cmp(a).unwrap());
                        top_three.truncate(3);
                        current_sum = 0;
                    },
                    _ => {
                        let num = text.parse::<i32>().unwrap();
                        current_sum += num;
                    }
                }
            },
            Err(e) => return Err(e.to_string())
        }
    }

    let top_three_sum = top_three.into_iter().reduce(|a, b| a + b);
    let result_part_2 = match top_three_sum {
        Some(result) => result,
        None => return Err(format!("No result for part two"))
    };

    Ok((total_max.to_string(), result_part_2.to_string()))
}
