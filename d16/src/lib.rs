use std::collections::HashMap;

#[derive(Debug)]
struct Valve {
    flow_rate: usize, // Pressure / minute
    tunnels: Vec<String>,
}

pub fn solve_d16(input_text: &mut String) -> usize {
    let valves = parse_input(input_text);
    let mut scores = solve_path_scores(valves);

    scores.sort_by(|a, b| b.cmp(a));
    return *scores.first().unwrap();
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
        let valve_flow_rate = part_1_parts.remove(0).parse::<usize>().unwrap();
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
                flow_rate: valve_flow_rate,
                tunnels,
            },
        );
    }

    println!("{:?}", valves);
    return valves;
}

fn depth_first_search(
    valve_name_current: String,
    path_score_current: usize,
    minutes_remaining: usize,
    valves: &HashMap<String, Valve>,
    valves_opened: Vec<String>,
    path_scores: &mut Vec<usize>,
) {
    let valve_current = valves.get(&valve_name_current).unwrap();
    let path_score_current = path_score_current;
    let mut valves_opened = valves_opened;

    // Check return condition
    if minutes_remaining == 0 || valves_opened.len() == valves.len() {
        path_scores.push(path_score_current);
        return;
    }

    // Check paths where pipe is opened first
    if valves_opened.contains(&valve_name_current) == false && valve_current.flow_rate != 0 {
        valves_opened.push(valve_name_current.clone());
        let path_score_new = path_score_current + (minutes_remaining - 1) * valve_current.flow_rate;
        depth_first_search(
            valve_name_current,
            path_score_new,
            minutes_remaining - 1,
            &valves,
            valves_opened.clone(),
            path_scores,
        )
    } else {
        // Check paths by iterating through each neighbour
        for t in &valve_current.tunnels {
            depth_first_search(
                t.clone(),
                path_score_current,
                minutes_remaining - 1,
                &valves,
                valves_opened.clone(),
                path_scores,
            )
        }
    }
}

fn solve_path_scores(valves: HashMap<String, Valve>) -> Vec<usize> {
    let mut path_scores: Vec<usize> = Vec::new();
    depth_first_search(
        String::from("AA"),
        0,
        30,
        &valves,
        Vec::new(),
        &mut path_scores,
    );

    // println!("{:?}", path_scores);
    return path_scores;
}

#[cfg(test)]
mod test {
    use super::*;

    const INPUT: &str = r#"Valve AA has flow rate=0; tunnels lead to valves DD, II, BB
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
        let max_pressure = solve_d16(&mut String::from(INPUT));
        assert_eq!(1651, max_pressure);
    }
}
