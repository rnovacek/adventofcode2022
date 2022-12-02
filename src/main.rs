use std::{env, path::Path, fs::File, io::BufReader};
mod d01;
mod d02;

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
        _ => {
            panic!("Unknown day: {}", day);
        }
    };

    match result {
        Ok(()) => {},
        Err(e) => panic!("Failed: {}", e),
    }

}
