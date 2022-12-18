use std::{io::{Read, BufReader, Bytes}, collections::{HashMap, BinaryHeap}, cmp::Ordering};

#[derive(Debug)]
struct Room {
    flow_rate: u32,
    exits: Vec<String>,
}

fn parse_rooms<R: Read>(input: &mut Bytes<BufReader<R>>) -> Result<HashMap<String, Room>, String> {
    let mut current_id: Option<String> = None;
    let mut current_flow_rate: Option<u32> = None;
    let mut current_exits: Vec<String> = Vec::new();

    let mut rooms: HashMap<String, Room> = HashMap::new();
    let mut current = String::with_capacity(7);
    for maybe_byte in input {
        match maybe_byte {
            Ok(b' ') | Ok(b'=') | Ok(b'\n') => {
                match current.as_str() {
                    "Valve" | "has" | "flow" | "rate" | "tunnel" | "tunnels" | "lead" | "leads" | "to" | "valve" | "valves" => {
                        current.clear();
                    },
                    _ => {
                        if current_id.is_none() {
                            current_id = Some(current);
                        } else if current_flow_rate.is_none() {
                            current_flow_rate = match current.parse::<u32>() {
                                Ok(rate) => Some(rate),
                                Err(_) => return Err(format!("Flow rate '{}' is not a number", current)),
                            }
                        } else {
                            current_exits.push(current);
                        }
                        current = String::new();
                    }
                }

                if maybe_byte.unwrap() == b'\n' {
                    let id = match current_id {
                        Some(x) => x,
                        None => return Err(String::from("No room id")),
                    };
                    let flow_rate = match current_flow_rate {
                        Some(x) => x,
                        None => return Err(String::from("No flow rate")),
                    };
                    rooms.insert(id, Room {
                        flow_rate,
                        exits: current_exits,
                    });
                    current_id = None;
                    current_flow_rate = None;
                    current_exits = Vec::new();
                }
            },
            Ok(b';') | Ok(b',') => {}, // ignore
            Ok(byte) => current.push(byte as char),
            Err(e) => return Err(format!("Invalid input: {}", e)),
        }
    }

    Ok(rooms)
}

#[derive(Debug)]
struct Node {
    room_ids: (String, String),
    remaining_times: (u32, u32),
    open_valves: Vec<String>,
    pressure_released: u32,
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> Ordering {
        self.pressure_released.cmp(&other.pressure_released)
    }
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Node {
    fn eq(&self, other: &Self) -> bool {
        self.room_ids.0 == other.room_ids.0 &&
        self.room_ids.1 == other.room_ids.1 &&
        self.remaining_times.0 == other.remaining_times.0 &&
        self.remaining_times.1 == other.remaining_times.1 &&
        self.open_valves == other.open_valves &&
        self.pressure_released == other.pressure_released
    }
}

impl Eq for Node {}

fn open_valves(rooms: &HashMap<String, Room>, distances: &DistanceMatrix, current: &Node, maybe_my_valve: Option<&String>, maybe_elephant_valve: Option<&String>) -> Option<Node> {
    // Simulate opening valve
    let mut new_node = Node {
        open_valves: current.open_valves.clone(),
        pressure_released: current.pressure_released,
        remaining_times: (current.remaining_times.0, current.remaining_times.1),
        room_ids: (current.room_ids.0.clone(), current.room_ids.1.clone()),
    };

    // My turn
    match maybe_my_valve {
        Some(my_valve) => {
            new_node.open_valves.push(my_valve.clone());
            new_node.room_ids = (my_valve.clone(), new_node.room_ids.1);

            let flow_rate = rooms.get(my_valve).unwrap().flow_rate;
            let distance = distances.get(&current.room_ids.0, my_valve).unwrap();
            if current.remaining_times.0 < distance + 1 {
                // Out of time
                return None;
            }
            new_node.remaining_times.0 -= distance + 1;
            new_node.pressure_released += new_node.remaining_times.0 * flow_rate;
        },
        None => {},
    }

    // Elephant turn
    match maybe_elephant_valve {
        Some(elephant_valve) => {
            new_node.open_valves.push(elephant_valve.clone());
            new_node.room_ids = (new_node.room_ids.0, elephant_valve.clone());

            let flow_rate = rooms.get(elephant_valve).unwrap().flow_rate;
            let distance = distances.get(&current.room_ids.1, elephant_valve).unwrap();

            if current.remaining_times.1 < distance + 1 {
                // Out of time
                return None;
            }

            new_node.remaining_times.1 -= distance + 1;
            new_node.pressure_released += new_node.remaining_times.1 * flow_rate;
        },
        None => {},
    }

    Some(new_node)
}

fn find_solution(start: Node, rooms: &HashMap<String, Room>, distances: &DistanceMatrix, elephant_moves: bool) -> u32 {
    let mut valves: Vec<&String> = Vec::new();
    for (id, room) in rooms.iter() {
        if room.flow_rate > 0 {
            valves.push(id);
        }
    }

    valves.sort_by_cached_key(|v| 1000 - rooms.get(*v).unwrap().flow_rate);

    let mut best_to_open: HashMap<String, u32> = HashMap::new();

    let mut max_pressure = 0u32;

    let mut heap: BinaryHeap<Node> = BinaryHeap::new();
    heap.push(start);
    loop {
        let current = match heap.pop() {
            Some(node) => node,
            None => break,
        };

        if current.pressure_released > max_pressure {
            max_pressure = current.pressure_released;
        }


        let mut open = current.open_valves.clone();
        open.sort();
        let key = open.join("-");
        match best_to_open.get(&key) {
            Some(best) => {
                if *best < current.pressure_released {
                    best_to_open.insert(key, current.pressure_released);
                } else {
                    continue;
                }
            },
            None => {
                best_to_open.insert(key, current.pressure_released);
            }
        }


        if current.open_valves.len() == valves.len() {
            continue;
        }

        for my_valve in valves.iter() {
            if current.open_valves.contains(*my_valve) {
                // Valve already open
                continue;
            }

            if elephant_moves {
                for elephant_valve in valves.iter() {
                    if current.open_valves.contains(*elephant_valve) {
                        // Valve already open
                        continue;
                    }
                    if my_valve == elephant_valve {
                        // Can't open the same valve
                        continue;
                    }


                    match open_valves(rooms, distances, &current, Some(my_valve), Some(elephant_valve)) {
                        Some(node) => heap.push(node),
                        None => {},
                    }

                    match open_valves(rooms, distances, &current, None, Some(elephant_valve)) {
                        Some(node) => heap.push(node),
                        None => {},
                    }
                }
            }


            // Elephant not moving
            match open_valves(rooms, distances, &current, Some(my_valve), None) {
                Some(node) => heap.push(node),
                None => {}
            }
        }
    }
    max_pressure
}

struct DistanceMatrix {
    distances: HashMap<String, u32>,
}

impl DistanceMatrix {
    fn new(rooms: &HashMap<String, Room>) -> Self {
        let mut matrix = DistanceMatrix { distances: HashMap::new() };

        for _ in 0..=rooms.len() {
            for (id, room) in rooms {
                for exit in room.exits.iter() {
                    matrix.set(id, exit, 1);
                    for other in rooms.keys() {
                        match matrix.get(&exit, other) {
                            Some(distance) => {
                                matrix.set(id, other, distance + 1);
                            },
                            None => {},
                        }
                    }
                }
            }
        }

        for room in rooms.keys() {
            matrix.set(room, room, 0);
        }

        matrix
    }

    fn set(&mut self, from: &String, to: &String, distance: u32) {
        let should_insert = match self.get(&from, &to) {
            Some(old_distance) => old_distance > distance + 1,
            None => true,
        };
        if should_insert {
            let key = format!("{}-{}", from, to);
            self.distances.insert(key, distance);
        }
    }

    fn get(&self, from: &String, to: &String) -> Option<u32> {
        let key = format!("{}-{}", from, to);
        match self.distances.get(&key) {
            Some(d) => Some(*d),
            None => {
                let reverse_key = format!("{}-{}", to, from);
                self.distances.get(&reverse_key).copied()
            },
        }
    }

    fn _print(&self, rooms: &HashMap<String, Room>) {
        print!("   ");
        for (id, room) in rooms.iter() {
            if room.flow_rate > 0 {
                print!(" {}", id);
            }
        }
        println!();

        for (id, room) in rooms.iter() {
            if room.flow_rate > 0 {
                print!("{} ", id);
                for (other_id, other) in rooms.iter() {
                    if other.flow_rate > 0 {
                        print!("{:3}", self.get(id, other_id).unwrap());
                    }
                }
                println!();
            }
        }
    }
}

pub fn run<R: Read>(input: BufReader<R>) -> Result<(String, String), String> {
    let rooms = parse_rooms(&mut input.bytes())?;

    let distances = DistanceMatrix::new(&rooms);

    // distances._print(&rooms);

    let start1 = Node {
        open_valves: Vec::new(),
        room_ids: (String::from("AA"), String::from("AA")),
        remaining_times: (30, 30),
        pressure_released: 0,
    };

    let result1 = find_solution(start1, &rooms, &distances, false);

    let start2 = Node {
        open_valves: Vec::new(),
        room_ids: (String::from("AA"), String::from("AA")),
        remaining_times: (26, 26),
        pressure_released: 0,
    };

    let result2 = find_solution(start2, &rooms, &distances, true);

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

    const SAMPLE: &str = "Valve AA has flow rate=0; tunnels lead to valves DD, II, BB
Valve BB has flow rate=13; tunnels lead to valves CC, AA
Valve CC has flow rate=2; tunnels lead to valves DD, BB
Valve DD has flow rate=20; tunnels lead to valves CC, AA, EE
Valve EE has flow rate=3; tunnels lead to valves FF, DD
Valve FF has flow rate=0; tunnels lead to valves EE, GG
Valve GG has flow rate=0; tunnels lead to valves FF, HH
Valve HH has flow rate=22; tunnel leads to valve GG
Valve II has flow rate=0; tunnels lead to valves AA, JJ
Valve JJ has flow rate=21; tunnel leads to valve II
";

    #[test]
    fn test_d16_distances() {
        let input = String::from(SAMPLE);
        let buf = str_to_buf_reader(&input);
        let rooms = parse_rooms(&mut buf.bytes()).expect("Unable to parse");

        let distances = DistanceMatrix::new(&rooms);
        // distances._print(&rooms);

        assert_eq!(distances.get(&String::from("HH"), &String::from("JJ")).unwrap(), 7);
        assert_eq!(distances.get(&String::from("JJ"), &String::from("HH")).unwrap(), 7);
    }

    #[test]
    fn test_d16_sample() {
        let input = String::from(SAMPLE);
        let buf = str_to_buf_reader(&input);
        let result = run(buf).expect("Run failed");

        assert_eq!(
            result.0,
            "1651",
            "Part 1 failed, value '{}' expected '1651'", result.0,
        );

        assert_eq!(
            result.1,
            "1707",
            "Part 2 failed, value '{}' expected '1707'", result.1,
        );
    }

    #[test]
    fn test_d16_final() {
        let f = std::fs::File::open("src/d16/input.txt").expect("No src/16/input.txt file");
        let result = run(BufReader::new(f)).expect("Run failed");
        assert_eq!(
            result.0,
            "1940",
        );
        assert_eq!(
            result.1,
            "2469",
        );
    }
}
