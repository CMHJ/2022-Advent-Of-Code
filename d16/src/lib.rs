/*!
D16 - Proboscidea Volcanium - Valve Pressure Problem.
Answer was taken from https://www.youtube.com/watch?v=bLMj50cpOug as I could
not work this out myself.

This is a graph problem however it cannot be traversed in the typical way.
The graph is too large to brute force, and in my first attempt blows the
stack.  The weights of each edge are changing with each step and needs to be
considered.  Valves that are blocked and have zero pressure released are of
no interest.

This graph need to be collapsed to only the nodes that release pressure
whilst maintaining the time it takes to travel between them.

An algorithm to solve this is the Floyd-Warshall algorithm for finding the
shortest paths in a directed weighted graph.  This is slightly inefficient
as we only need to know the shortest path between some of the nodes
discounting the nodes with 0 flow rate.

FW is O(n^3) but breadth first search is O(V + E) or number of vertices plus
edges, which apparently can be worse than FW but on average will be better.
*/

#[cfg(test)]
mod test;

use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::VecDeque;
use std::cmp::max;

const TIME_REMAINING_P1_START: usize = 30;

type ValveMap = HashMap<String, Valve>;
type ValveDistanceMap = HashMap<String, usize>;
type ValveCache = HashMap<(usize, String, usize), usize>;

struct Valve {
    flow_rate: usize,
    tunnels: Vec<String>,
}

struct ValveData {
    valves: ValveMap,
    valve_distance_maps: HashMap<String, ValveDistanceMap>,
    valve_indices: HashMap<String, usize>
    // valve_cache:
}

pub fn solve_p1(input: String) -> usize {
    let valves: ValveMap = parse_input(input);
    let valve_data = simplify_graph(valves);
    let mut valve_cache = ValveCache::new();
    let max_score = dfs(&valve_data, &mut valve_cache, TIME_REMAINING_P1_START, "AA".to_string(), 0);

    max_score
}

fn parse_input(input: String) -> ValveMap {
    let mut valves: HashMap<String, Valve> = HashMap::new();
    for line in input.lines() {
        let valve_name = line
            .split_ascii_whitespace()
            .nth(1)
            .expect("Name parse: Could not get valve name");
        let valve_name = String::from(valve_name);
        let flow_rate = line
            .split(';')
            .nth(0)
            .expect("Flowrate parse: could not get 0th part of ; split")
            .split('=')
            .nth(1)
            .expect("Flowrate parse: could not get 1st part of = split")
            .parse::<usize>()
            .expect("Flowrate parse: could not parse as usize");

        let tunnels: Vec<String> = line
            .split_once("to ")
            .expect("Tunnel parse: Could not split at \"to \"")
            .1
            .split_once(' ')
            .expect("Tunnel parse: Could not split at spaces")
            .1
            .split(", ")
            .map(|s| String::from(s))
            .collect();

        // println!("{}, {}, {:?}", valve_name, flow_rate, tunnels);
        valves.insert(valve_name, Valve { flow_rate, tunnels });
    }

    valves
}

fn simplify_graph(valves: ValveMap) -> ValveData {
    /*! Perform BFS to simplify the graph removing valves with a flow rate of 0
    and keeping track of the total distances between valves of interest.
    */

    // Map of distance from each 0 flow rate node to every non-0 node.
    // distance is in minutes
    let mut valve_distance_maps: HashMap<String, ValveDistanceMap> = HashMap::new();
    let mut valves_of_interest: Vec<String> = Vec::with_capacity(valves.len());

    // BFS starting from every 0 flow rate valve to build the map
    for (valve_name, v) in &valves {
        // Ignore valves with no flowrate but not AA as it is the starting node
        if valve_name != "AA" && v.flow_rate == 0 {
            continue;
        }

        if valve_name != "AA" {
            valves_of_interest.push(valve_name.clone());
        }

        let valve_map_init = HashMap::from([(valve_name.clone(), 0), ("AA".to_string(), 0)]);
        valve_distance_maps.insert(valve_name.clone(), valve_map_init);

        let mut visited: HashSet<String> = HashSet::from([valve_name.clone()]);
        let mut queue: VecDeque<(usize, String)> = VecDeque::from([(0, valve_name.clone())]);

        while let Some((distance, position)) = queue.pop_front() {
            for neighbour in &valves.get(&position).unwrap().tunnels {
                // If there is already an entry ignore or mark is visited and continue
                if visited.contains(neighbour) {
                    continue;
                }
                visited.insert(neighbour.clone());

                // Collect neighbour if flow rate is non-0
                if valves.get(neighbour).unwrap().flow_rate != 0 {
                    valve_distance_maps
                        .entry(valve_name.clone())
                        .or_insert(HashMap::new())
                        .insert(neighbour.clone(), distance + 1);
                }

                // Queue neighbour to be explored + distance to neighbour
                queue.push_back((distance + 1, neighbour.clone()));
            }
        }

        valve_distance_maps.get_mut(valve_name).unwrap().remove(valve_name);
        if valve_name != "AA" {
            valve_distance_maps.get_mut(valve_name).unwrap().remove("AA");
        }

        // println!("{:#?}", valve_distance_maps);
    }

    // Generate valve indices hashmap to map valve name to position in bitmask
    let mut valve_indices: HashMap<String, usize> = HashMap::new();
    for (i, valve_name) in valves_of_interest.iter().enumerate() {
        valve_indices.insert(valve_name.clone(), i);
    }

    ValveData {
        valves,
        valve_distance_maps,
        valve_indices
    }
}

fn dfs(valve_data: &ValveData, valve_cache: &mut ValveCache, time_remaining: usize, valve_name_current: String, valves_open_bitmask: usize) -> usize {
    let cache_key = &(time_remaining, valve_name_current.clone(), valves_open_bitmask);
    if valve_cache.contains_key(cache_key) {
        return *valve_cache.get(cache_key).unwrap();
    }

    let mut max_score: usize = 0;
    for neighbour in valve_data.valve_distance_maps.get(&valve_name_current).unwrap().keys() {
        // Check if valve is already open in the bitmask
        let bit: usize = 1 << valve_data.valve_indices.get(neighbour).unwrap();
        if valves_open_bitmask & bit != 0 {
            continue;
        }

        // Calculate if time remaining doesn't go past zero
        let time_remaining_after_move: isize = time_remaining as isize - (1 + valve_data.valve_distance_maps.get(&valve_name_current).unwrap().get(neighbour).unwrap()) as isize;
        if time_remaining_after_move <= 0 {
            continue;
        }
        let time_remaining_after_move: usize = time_remaining_after_move as usize;

        max_score = max(
            max_score,
            dfs(
                valve_data,
                valve_cache,
                time_remaining_after_move,
                neighbour.clone(),
                valves_open_bitmask | bit // Open valve in bitmask
            ) + valve_data.valves.get(neighbour).unwrap().flow_rate * time_remaining_after_move as usize
        );
    }

    max_score
}