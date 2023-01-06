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

pub enum GameResult {
    Lose,
    Draw,
    Win,
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

fn eval_code_2(c: &char) -> GameResult {
    match c {
        'X' => GameResult::Lose,
        'Y' => GameResult::Draw,
        'Z' => GameResult::Win,
        _ => panic!("Result code not found"),
    }
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

fn eval_predetermined_ending(a: &RPS, res: &GameResult) -> RPS {
    match a {
        RPS::Rock => match res {
            GameResult::Win => RPS::Paper,
            GameResult::Lose => RPS::Scissors,
            GameResult::Draw => RPS::Rock,
        },
        RPS::Paper => match res {
            GameResult::Win => RPS::Scissors,
            GameResult::Lose => RPS::Rock,
            GameResult::Draw => RPS::Paper,
        },
        RPS::Scissors => match res {
            GameResult::Win => RPS::Rock,
            GameResult::Lose => RPS::Paper,
            GameResult::Draw => RPS::Scissors,
        }
    }
}

fn eval_outcome(a: &RPS, b: &RPS) -> GameResult {
    match a {
        RPS::Rock => match b {
            RPS::Rock => GameResult::Draw,
            RPS::Paper => GameResult::Win,
            RPS::Scissors => GameResult::Lose,
        },
        RPS::Paper => match b {
            RPS::Rock => GameResult::Lose,
            RPS::Paper => GameResult::Draw,
            RPS::Scissors => GameResult::Win,
        },
        RPS::Scissors => match b {
            RPS::Rock => GameResult::Win,
            RPS::Paper => GameResult::Lose,
            RPS::Scissors => GameResult::Draw,
        }
    }
}

pub fn eval_result(a: &char, b: &char) -> u8 {
    let a_eval = eval_code(&a);
    let b_eval = eval_code(&b);

    let res = eval_outcome(&a_eval, &b_eval);

    let res_score = eval_score(&res);
    let choice_score = eval_code_score(&b_eval);

    res_score + choice_score
}

pub fn eval_result_2(a: &char, b: &char) -> u8 {
    let a_eval = eval_code(&a);
    let b_eval = eval_code_2(&b);
    let b_choice = eval_predetermined_ending(&a_eval, &b_eval);
    let res = eval_outcome(&a_eval, &b_choice);

    let res_score = eval_score(&res);
    let choice_score = eval_code_score(&b_choice);

    res_score + choice_score
}
