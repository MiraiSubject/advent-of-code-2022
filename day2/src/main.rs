use std::fs;
#[derive(Debug)]
// Rock = A | X, Paper = B | Y, Scissors = C | Z
// Rock = 1, Paper = 2, Scissors = 3
enum RockPaperScissors {
    Rock,
    Paper,
    Scissors,
}
#[derive(Debug)]
// Win = 6, Tie = 3, Lose = 0
enum Results {
    Win,
    Tie,
    Lose,
}

impl RockPaperScissors {}

fn main() {
    let contents = fs::read_to_string("./input.txt").expect("File not found");
    let mut total = 0;
    for matches in contents.split("\n") {
        // let mut results = vec![];

        // for actions in matches.split(" ") {
        //     let x = match actions {
        //         "A" | "X" => RockPaperScissors::Rock,
        //         "B" | "Y" => RockPaperScissors::Paper,
        //         "C" | "Z" => RockPaperScissors::Scissors,
        //         _ => todo!(),
        //     };
        //     results.push(x);
        // }

        let mut actions = matches.split(" ");
        let opponentsMove = actions.next().unwrap();
        let result = actions.next().unwrap();

        let opponentsMove = match opponentsMove {
            "A" => RockPaperScissors::Rock,
            "B" => RockPaperScissors::Paper,
            "C" => RockPaperScissors::Scissors,
            _ => panic!(),
        };

        let result = match result {
            "X" => Results::Lose,
            "Y" => Results::Tie,
            "Z" => Results::Win,
            _ => panic!(),
        };

        let pick = match opponentsMove {
            RockPaperScissors::Rock => match result {
                Results::Win => RockPaperScissors::Paper,
                Results::Tie => RockPaperScissors::Rock,
                Results::Lose => RockPaperScissors::Scissors,
            },
            RockPaperScissors::Paper => match result {
                Results::Win => RockPaperScissors::Scissors,
                Results::Tie => RockPaperScissors::Paper,
                Results::Lose => RockPaperScissors::Rock,
            },
            RockPaperScissors::Scissors => match result {
                Results::Win => RockPaperScissors::Rock,
                Results::Tie => RockPaperScissors::Scissors,
                Results::Lose => RockPaperScissors::Paper,
            },
        };

        let matchResult = match pick {
            RockPaperScissors::Rock => {
                1 + match opponentsMove {
                    RockPaperScissors::Rock => 3,
                    RockPaperScissors::Paper => 0,
                    RockPaperScissors::Scissors => 6,
                }
            }
            RockPaperScissors::Paper => {
                2 + match opponentsMove {
                    RockPaperScissors::Rock => 6,
                    RockPaperScissors::Paper => 3,
                    RockPaperScissors::Scissors => 0,
                }
            }
            RockPaperScissors::Scissors => {
                3 + match opponentsMove {
                    RockPaperScissors::Rock => 0,
                    RockPaperScissors::Paper => 6,
                    RockPaperScissors::Scissors => 3,
                }
            }
        };
        total += matchResult;
    }
    println!("{total}");
}
