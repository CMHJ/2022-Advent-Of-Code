/*
 * Scores
 * 0 - Lose
 * 3 - Draw
 * 6 - Win
 */

/*
 * Codes
 * A or X - Rock
 * B or Y - Paper
 * C or Z - Scissors
 */

enum RPS {
    Rock,
    Paper,
    Scissors,
}

fn eval_code(c: &char) -> RPS {
    match c {
        'A' => RPS::Rock,
        'X' => RPS::Rock,
        'B' => RPS::Paper,
        'Y' => RPS::Paper,
        'C' => RPS::Scissors,
        'Z' => RPS::Scissors,
        _ => panic!("RPS code not found"),
    }
}

pub enum GameResult {
    Lose,
    Draw,
    Win,
}

fn eval_code_score(a: &RPS) -> u8 {
    match a {
        RPS::Rock => 1,
        RPS::Paper => 2,
        RPS::Scissors => 3,
    }
}

fn eval_score(res: &GameResult) -> u8 {
    match res {
        GameResult::Lose => 0,
        GameResult::Draw => 3,
        GameResult::Win => 6,
    }
}

pub fn eval_result(a: &char, b: &char) -> u8 {
    let a_eval = eval_code(&a);
    let b_eval = eval_code(&b);

    let res = match a_eval {
        RPS::Rock => match b_eval {
            RPS::Rock => GameResult::Draw,
            RPS::Paper => GameResult::Win,
            RPS::Scissors => GameResult::Lose,
        },
        RPS::Paper => match b_eval {
            RPS::Rock => GameResult::Lose,
            RPS::Paper => GameResult::Draw,
            RPS::Scissors => GameResult::Win,
        },
        RPS::Scissors => match b_eval {
            RPS::Rock => GameResult::Win,
            RPS::Paper => GameResult::Lose,
            RPS::Scissors => GameResult::Draw,
        }
    };

    let res_score = eval_score(&res);
    let choice_score = eval_code_score(&b_eval);

    res_score + choice_score
}
