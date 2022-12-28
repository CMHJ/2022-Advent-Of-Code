use std::collections::HashMap;
use std::fs;

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

fn parse_input(s: &mut String) {
    // Split into valve flow rates and paths
    // Insert flow rate into hashmap using valve name as key
    // Insert tunnels into hashmap using valve name as key
    // Struct containing flowrate and tunnels?

    // Delete sections of the line
    *s = s.replace("Valve ", "");
    *s = s.replace("has flow rate=", "");
    *s = s.replace("has flow rate=", "");

    let mut valves: HashMap<&str, Valve> = HashMap::new();
    for line in s.split('\n') {
        // Split each line based on semi-colon
        let line_parts: Vec<String> = line
            .split("; ")
            .map(|string| String::from(string))
            .collect();

        let line_part_1 = &line_parts[0];
        let line_part_2 = &line_parts[1];

        // Process first part
        let mut part_1_parts: Vec<String> = line_part_1
            .split(' ')
            .map(|string| String::from(string))
            .collect();
        let valve_name  = part_1_parts.remove(0);
        let valve_flowrate = part_1_parts.remove(0).parse::<u64>().unwrap();

        // Process second part
    }
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
        parse_input(s);
    }
}
