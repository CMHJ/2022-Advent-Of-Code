#[cfg(test)]
mod test;

pub const PART_1_MARKER_LEN: usize = 4;
pub const PART_2_MARKER_LEN: usize = 14;


pub fn solve(input: String, marker_len: usize) -> usize {
    // History of different chars
    let mut char_hist: Vec<char> = Vec::with_capacity(marker_len);
    let mut cnt: usize = 0; // Count number of different characters so far

    // Iterate over characters
    for (i, c) in input.chars().enumerate() {
        // loop through character history
        // if char matches pop history until the match
        let mut c_match = false;
        for (j, c_hist) in char_hist.iter().enumerate() {
            if c == *c_hist {
                c_match = true;
                for _ in 0..=j {
                    char_hist.remove(0);
                }
                break;
            }
        }
        // push onto history after check
        char_hist.push(c);

        // If a match was found reset the counter and continue
        if c_match == true {
            cnt = char_hist.len();
            continue;
        }

        cnt += 1;
        if cnt >= marker_len {
            let pos = i + 1;
            println!("Found SOP marker at position {}!", pos);
            return pos;
        }
    }

    panic!("Start-of-packet marker could not be found!");
}
