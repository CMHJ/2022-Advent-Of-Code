use std::{env, fs, process::exit};

fn main() {
    let args: Vec<String> = env::args().collect();
    // println!("{:?} {}", args, args.len());
    if args.len() == 1 || args.len() > 2 {
        eprintln!("Please supply path to input text");
        exit(1);
    }

    let path = args.get(1).unwrap();

    // Read file
    let contents = fs::read_to_string(path).unwrap();
    // println!("{:#?}", contents);

    // Split text to get groups
    let groups: Vec<&str> = contents.split("\n\n").collect();
    // println!("{:?}", groups);

    // Split again to get individual numbers
    // Find highest number
    let mut totals: Vec<u64> = Vec::new();
    totals.reserve_exact(groups.len());

    for g in groups {
        let mut total = 0;
        for n in g.split('\n') {
            total += n.parse::<u64>().unwrap();
        }

        totals.push(total);
    }

    totals.sort_by(|a, b| b.cmp(a));

    // Print result
    let highest = &totals[0];
    println!("Highest number of calories is: {}", highest);
    let mut highest_3: u64 = 0;
    for x in 0..=2 {
        highest_3 += totals[x];
    }
    println!("Highest number of calories for top 3: {}", highest_3);
}
