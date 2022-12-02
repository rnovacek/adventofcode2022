use std::fs::File;
use std::io::{ BufReader, BufRead };

pub fn run(input: BufReader<File>) -> Result<(), String> {
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

    println!("Result part one: {}", total_max);
    let top_three_sum = top_three.into_iter().reduce(|a, b| a + b);
    match top_three_sum {
        Some(result) => println!("Result part two: {}", result),
        None => println!("No result for part two")
    }


    Ok(())
}
