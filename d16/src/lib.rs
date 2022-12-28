use std::collections::HashMap;
use std::fs;

#[derive(Debug)]
struct Valve {
    flow_rate: u64,
    tunnels: Vec<String>,
}

pub fn solve_d16(input_file_path: &String) {
    let mut contents = read_input_file(input_file_path);
    parse_input(&mut contents);
}

fn read_input_file(path: &String) -> String {
    fs::read_to_string(path).unwrap()
}

fn parse_input(input: &mut String) -> HashMap<String, Valve> {
    // Split into valve flow rates and paths
    // Insert flow rate into hashmap using valve name as key
    // Insert tunnels into hashmap using valve name as key
    // Struct containing flowrate and tunnels?

    // Delete sections of the line
    *input = input.replace("Valve ", "");
    *input = input.replace("has flow rate=", "");
    *input = input.replace("has flow rate=", "");

    // Initialise HashMap with capacity for each entry
    let mut valves: HashMap<String, Valve> = HashMap::with_capacity(input.lines().count());

    // Initialise regex for use later
    let tunnel_pattern =
        regex::Regex::new(r"tunnels? leads? to valves? ").expect("Could not create regex");

    for line in input.split('\n') {
        // Split each line based on semi-colon
        // let mut line_parts: Vec<String> = line.split("; ").map(|s| s.to_string()).collect();
        let (line_part_1, mut line_part_2) = line
            .split_once("; ")
            .map(|(s1, s2)| (s1.to_string(), s2.to_string()))
            .expect("Could not split line into two strings");

        // let line_part_1 = line_parts.remove(0);
        // let mut line_part_2 = line_parts.remove(0);

        // Process first part
        let mut part_1_parts: Vec<String> = line_part_1.split(' ').map(|s| s.to_string()).collect();
        let valve_name = part_1_parts.remove(0);
        let valve_flowrate = part_1_parts.remove(0).parse::<u64>().unwrap();
        // println!("{} {}", valve_name, valve_flowrate);

        // Process second part
        // Extract valve tunnels
        let pattern_matches = tunnel_pattern
            .captures(&line_part_2)
            .expect("Found no matches");
        let pattern_match_location = pattern_matches
            .get(0)
            .expect("Unable to retrieve match location");
        line_part_2.replace_range(
            pattern_match_location.start()..pattern_match_location.end(),
            "",
        );

        // Split by commas
        let tunnels: Vec<String> = line_part_2.split(", ").map(|s| s.to_string()).collect();

        // Add to HashMap
        valves.insert(
            valve_name,
            Valve {
                flow_rate: valve_flowrate,
                tunnels: tunnels,
            },
        );
    }

    println!("{:#?}", valves);
    return valves;
}

#[cfg(test)]
mod test {
    use super::*;

    const input: &str = r#"Valve AA has flow rate=0; tunnels lead to valves DD, II, BB
Valve BB has flow rate=13; tunnels lead to valves CC, AA
Valve CC has flow rate=2; tunnels lead to valves DD, BB
Valve DD has flow rate=20; tunnels lead to valves CC, AA, EE
Valve EE has flow rate=3; tunnels lead to valves FF, DD
Valve FF has flow rate=0; tunnels lead to valves EE, GG
Valve GG has flow rate=0; tunnels lead to valves FF, HH
Valve HH has flow rate=22; tunnel leads to valve GG
Valve II has flow rate=0; tunnels lead to valves AA, JJ
Valve JJ has flow rate=21; tunnel leads to valve II"#;

    #[test]
    fn test_example() {
        parse_input(&mut String::from(input));
    }
}
