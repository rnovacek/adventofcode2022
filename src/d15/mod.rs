use std::{io::{Read, BufReader, Bytes}, cmp};

struct Sensor {
    x: isize,
    y: isize,
    range: isize,
}

struct Beacon {
    x: isize,
    y: isize,
}

fn manhattan_distance(x1: isize, y1: isize, x2: isize, y2: isize) -> isize {
    (x1.abs_diff(x2) + y1.abs_diff(y2)) as isize
}

fn read_sensors<R: Read>(input: &mut Bytes<BufReader<R>>) -> Result<(Vec<Sensor>, Vec<Beacon>), String> {
    let mut sensors: Vec<Sensor> = Vec::new();
    let mut beacons: Vec<Beacon> = Vec::new();
    let mut current = String::with_capacity(4);

    let mut positions: Vec<isize> = Vec::with_capacity(4);

    for maybe_byte in input {
        match maybe_byte {
            Ok(ch) if ch >= b'0' && ch <= b'9' => {
                current.push(ch as char);
            },
            Ok(b'-') => {
                current.push('-');
            },
            Ok(_) => {
                if current.len() > 0 {
                    positions.push(match current.parse::<isize>() {
                        Ok(n) => n,
                        Err(e) => return Err(format!("Number error: {}", e)),
                    })
                }
                current.clear();

                if maybe_byte.unwrap() == b'\n' {
                    let range = manhattan_distance(positions[0], positions[1], positions[2], positions[3]);
                    sensors.push(Sensor { x: positions[0], y: positions[1], range });
                    beacons.push(Beacon { x: positions[2], y: positions[3] });
                    positions.clear();
                }
            },
            Err(e) => return Err(format!("Unable to read: {}", e)),
        }
    }
    Ok((sensors, beacons))
}


fn count_empty_at_row(sensors: &Vec<Sensor>, beacons: &Vec<Beacon>, check_at_row: isize) -> usize {
    let mut result = 0usize;

    let mut min_x: isize = isize::MAX;
    let mut max_x: isize = isize::MIN;

    for sensor in sensors {
        min_x = cmp::min(min_x, sensor.x - sensor.range);
        max_x = cmp::max(max_x, sensor.x + sensor.range);
    }

    for x in min_x..=max_x {
        let mut is_empty = true;
        for sensor in sensors {
            let distance = manhattan_distance(sensor.x, sensor.y, x, check_at_row);
            if distance <= sensor.range {
                is_empty = false;
            }
        }
        for beacon in beacons {
            if beacon.x == x && beacon.y == check_at_row {
                is_empty = true;
            }
        }
        if !is_empty {
            result += 1;
        } else {
        }
    }
    println!();

    result
}

fn find_empty_space(sensors: &Vec<Sensor>, start: isize, end: isize) -> Option<isize> {
    for y in start..=end {
        let mut x = start;
        while x <= end {
            let mut is_empty = true;
            for sensor in sensors {
                let distance = manhattan_distance(sensor.x, sensor.y, x, y);
                if distance <= sensor.range {
                    is_empty = false;
                    let y_diff = sensor.y.abs_diff(y) as isize;
                    x = sensor.x + (sensor.range.abs_diff(y_diff) as isize) + 1;
                    break;
                }
            }

            if is_empty {
                return Some(x * 4_000_000 + y);
            }
        }
    }
    return None;
}

pub fn run<R: Read>(input: BufReader<R>) -> Result<(String, String), String> {
    let (sensors, beacons) = read_sensors(&mut input.bytes())?;


    let result1 = count_empty_at_row(&sensors, &beacons, 2_000_000);
    let result2 = match find_empty_space(&sensors, 0, 4_000_000) {
        Some(x) => x,
        None => return Err(format!("No empty space found")),
    };

    Ok((
        format!("{}", result1),
        format!("{}", result2),
    ))
}

#[cfg(test)]
mod test {
    use std::io::BufReader;

    use super::*;
    use crate::test_util::str_to_buf_reader;

    #[test]
    fn test_d15_01() {
        let input = String::from("Sensor at x=2, y=18: closest beacon is at x=-2, y=15
Sensor at x=9, y=16: closest beacon is at x=10, y=16
Sensor at x=13, y=2: closest beacon is at x=15, y=3
Sensor at x=12, y=14: closest beacon is at x=10, y=16
Sensor at x=10, y=20: closest beacon is at x=10, y=16
Sensor at x=14, y=17: closest beacon is at x=10, y=16
Sensor at x=8, y=7: closest beacon is at x=2, y=10
Sensor at x=2, y=0: closest beacon is at x=2, y=10
Sensor at x=0, y=11: closest beacon is at x=2, y=10
Sensor at x=20, y=14: closest beacon is at x=25, y=17
Sensor at x=17, y=20: closest beacon is at x=21, y=22
Sensor at x=16, y=7: closest beacon is at x=15, y=3
Sensor at x=14, y=3: closest beacon is at x=15, y=3
Sensor at x=20, y=1: closest beacon is at x=15, y=3
");
        let buf = str_to_buf_reader(&input);
        let (sensors, beacons) = read_sensors(&mut buf.bytes()).expect("Run failed");

        let result1 = count_empty_at_row(&sensors, &beacons, 10);
        assert_eq!(
            result1,
            26,
        );

        let result2 = find_empty_space(&sensors, 0, 20);
        assert_eq!(
            result2,
            Some(56000011),
        );


    }

    #[test]
    fn test_d15_final() {
        let f = std::fs::File::open("src/d15/input.txt").expect("No src/15/input.txt file");
        let result = run(BufReader::new(f)).expect("Run failed");
        assert_eq!(
            result.0,
            "5525990",
        );
        assert_eq!(
            result.1,
            "11756174628223",
        );
    }
}
