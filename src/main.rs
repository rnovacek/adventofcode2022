use std::{env, path::Path, fs::File, io::BufReader};
mod d01;
mod d02;
mod d03;
mod d04;
mod d05;
mod d06;
mod d07;
mod d08;
mod d09;
mod d10;
mod d11;
mod d12;
mod d13;

fn main() {
    let args: Vec<String> = env::args().collect();
    let day: String;
    let input: &Path;
    let filename: String;

    match args.len() {
        2 => {
            day = args[1].clone();
            filename = format!("src/d{}/input.txt", day);
            input = Path::new(&filename);
        },
        3.. => {
            day = args[1].clone();
            input = Path::new(&args[2]);
        },
        _ => {
            panic!("Usage: {} day", args[0]);
        }
    };

    let file = match File::open(input) {
        Ok(f) => f,
        Err(e) => panic!("Unable to open file {}: {}", input.display(), e.to_string()),
    };
    let reader = BufReader::new(file);

    let result = match day.as_str() {
        "01" => d01::run(reader),
        "02" => d02::run(reader),
        "03" => d03::run(reader),
        "04" => d04::run(reader),
        "05" => d05::run(reader),
        "06" => d06::run(reader),
        "07" => d07::run(reader),
        "08" => d08::run(reader),
        "09" => d09::run(reader),
        "10" => d10::run(reader),
        "11" => d11::run(reader),
        "12" => d12::run(reader),
        "13" => d13::run(reader),
        _ => {
            panic!("Unknown day: {}", day);
        }
    };

    match result {
        Ok((part1, part2)) => {
            println!("Result part one: {}", part1);
            println!("Result part two: {}", part2);
        },
        Err(e) => panic!("Failed: {}", e),
    }

}

#[cfg(test)]
pub mod test_util {
    use std::io::BufReader;

    pub fn str_to_buf_reader<'input>(input: &'input String) -> BufReader<&'input [u8]> {
        let b = input.as_bytes();
        return BufReader::new(b);
    }
}
