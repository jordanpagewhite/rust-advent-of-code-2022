#![allow(unused_variables)]
/*
--- Day 2: Rock Paper Scissors ---
The Elves begin to set up camp on the beach. To decide whose tent gets to be closest to the snack storage, a giant Rock Paper Scissors tournament is already in progress.

Rock Paper Scissors is a game between two players. Each game contains many rounds; in each round, the players each simultaneously choose one of Rock, Paper, or Scissors using a hand shape. Then, a winner for that round is selected: Rock defeats Scissors, Scissors defeats Paper, and Paper defeats Rock. If both players choose the same shape, the round instead ends in a draw.

Appreciative of your help yesterday, one Elf gives you an encrypted strategy guide (your puzzle input) that they say will be sure to help you win. "The first column is what your opponent is going to play: A for Rock, B for Paper, and C for Scissors. The second column--" Suddenly, the Elf is called away to help with someone's tent.

The second column, you reason, must be what you should play in response: X for Rock, Y for Paper, and Z for Scissors. Winning every time would be suspicious, so the responses must have been carefully chosen.

The winner of the whole tournament is the player with the highest score. Your total score is the sum of your scores for each round. The score for a single round is the score for the shape you selected (1 for Rock, 2 for Paper, and 3 for Scissors) plus the score for the outcome of the round (0 if you lost, 3 if the round was a draw, and 6 if you won).

Since you can't be sure if the Elf is trying to help you or trick you, you should calculate the score you would get if you were to follow the strategy guide.

For example, suppose you were given the following strategy guide:

A Y
B X
C Z
This strategy guide predicts and recommends the following:

In the first round, your opponent will choose Rock (A), and you should choose Paper (Y). This ends in a win for you with a score of 8 (2 because you chose Paper + 6 because you won).
In the second round, your opponent will choose Paper (B), and you should choose Rock (X). This ends in a loss for you with a score of 1 (1 + 0).
The third round is a draw with both players choosing Scissors, giving you a score of 3 + 3 = 6.
In this example, if you were to follow the strategy guide, you would get a total score of 15 (8 + 1 + 6).

What would your total score be if everything goes exactly according to your strategy guide?
*/

const POINTS_WIN: u8 = 6;
const POINTS_DRAW: u8 = 3;
const POINTS_LOSS: u8 = 0;

const POINTS_ROCK: u8 = 1;
const POINTS_PAPER: u8 = 2;
const POINTS_SCISSORS: u8 = 3;

pub fn solve() -> (u32, u32) {
    let input = include_str!("../input/day2/part1.txt");
    (part1(input), part2(input))
}

pub fn part2(input: &str) -> u32 {
    let mut total_score: u32 = 0;

    for mut line in input.lines() {
        let win_state = line.replace(" ", "").chars().last().unwrap();
        line = line.trim_end_matches(win_state);
        let opponent = line.replace(" ", "").chars().last().unwrap();

        total_score += get_part2_round_score(opponent, win_state) as u32;
    }

    total_score

}

pub fn part1(input: &str) -> u32 {
    let mut total_score: u32 = 0;

    for mut line in input.lines() {
        let player = line.replace(" ", "").chars().last().unwrap();
        line = line.trim_end_matches(player);
        let opponent = line.replace(" ", "").chars().last().unwrap();

        total_score += get_part1_round_score(opponent, player) as u32;
    }

    total_score
}

pub fn get_part1_round_score(opponent: char, player: char) -> u8 {
    let player_shape = get_player_shape(player);
    get_player_win_score(&get_opponent_shape(opponent), &player_shape) + get_shape_score(&player_shape)
}

pub fn get_part2_round_score(opponent: char, win_state: char) -> u8 {
    let player_shape = get_player_move_from_win_state(&get_opponent_shape(opponent), get_win_state_from_second_column(win_state));
    get_player_win_score(&get_opponent_shape(opponent), &player_shape) + get_shape_score(&player_shape)
}


pub fn get_player_win_score(opponent: &str, player: &str) -> u8 {
    return match (opponent, player) {
        ("rock", "rock") => POINTS_DRAW,
        ("rock", "paper") => POINTS_WIN,
        ("rock", "scissors") => POINTS_LOSS,
        ("paper", "rock") => POINTS_LOSS,
        ("paper", "paper") => POINTS_DRAW,
        ("paper", "scissors") => POINTS_WIN,
        ("scissors", "rock") => POINTS_WIN,
        ("scissors", "paper") => POINTS_LOSS,
        ("scissors", "scissors") => POINTS_DRAW,
        _ => unreachable!()
    }
}

pub fn get_player_move_from_win_state(opponent: &str, win_state: u8) -> String {
    return match (opponent, win_state) {
        ("rock", POINTS_DRAW) => String::from("rock"),
        ("rock", POINTS_WIN) => String::from("paper"),
        ("rock", POINTS_LOSS) => String::from("scissors"),
        ("paper", POINTS_LOSS) => String::from("rock"),
        ("paper", POINTS_DRAW) => String::from("paper"),
        ("paper", POINTS_WIN) => String::from("scissors"),
        ("scissors", POINTS_WIN) => String::from("rock"),
        ("scissors", POINTS_LOSS) => String::from("paper"),
        ("scissors", POINTS_DRAW) => String::from("scissors"),
        _ => unreachable!()
    }
}

pub fn get_opponent_shape(c: char) -> String {
    return match c {
        'A' => String::from("rock"),
        'B' => String::from("paper"),
        'C' => String::from("scissors"),
        _ => unreachable!()
    }
}

pub fn get_player_shape(c: char) -> String {
    return match c {
        'X' => String::from("rock"),
        'Y' => String::from("paper"),
        'Z' => String::from("scissors"),
        _ => unreachable!()
    }
}

pub fn get_win_state_from_second_column(c: char) -> u8 {
    return match c {
        'X' => POINTS_LOSS,
        'Y' => POINTS_DRAW,
        'Z' => POINTS_WIN,
        _ => unreachable!()
    }
}

pub fn get_shape_score(shape: &str) -> u8 {
    return match shape {
        "rock" => POINTS_ROCK,
        "paper" => POINTS_PAPER,
        "scissors" => POINTS_SCISSORS,
        _ => unreachable!()
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_player_win_score_win() {
        assert_eq!(get_player_win_score("rock", "paper"), POINTS_WIN);
    }

    #[test]
    fn test_get_player_win_score_draw() {
        assert_eq!(get_player_win_score("paper", "paper"), POINTS_DRAW);
    }
    
    #[test]
    fn test_get_player_win_score_loss() {
        assert_eq!(get_player_win_score("scissors", "paper"), POINTS_LOSS);
    }

    #[test]
    fn test_get_part1_round_score() {
        assert_eq!(get_part1_round_score('A', 'Y'), 8);
        assert_eq!(get_part1_round_score('B', 'X'), 1);
        assert_eq!(get_part1_round_score('C', 'Z'), 6);
    }

    #[test]
    fn test_part1() {
        let input = "A Y
B X
C Z";
        assert_eq!(part1(input), 15);
    }

    #[test]
    fn test_part2() {
        let input = "A Y
B X
C Z";
        assert_eq!(part2(input), 12);
    }


    #[test]
    fn test_solve() {
        assert_eq!(solve(), (8392, 10116));
    }
}
