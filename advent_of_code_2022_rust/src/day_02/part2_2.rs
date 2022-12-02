pub mod part2_2{
    use part2_1::part2_1::RPSChoice;
    use part2_1::part2_1::MatchResult;

    #[path = "../part2_1.rs"] mod part2_1;
    impl RPSChoice{
        
        pub fn get_move_for_match_result(opponent: &RPSChoice, match_result: &MatchResult) -> RPSChoice{
            match match_result{
                MatchResult::PlayerWins => {
                    match opponent {
                        RPSChoice::Rock => {
                            RPSChoice::Paper
                        },
                        RPSChoice::Paper => {
                            RPSChoice::Scissors
                        }
                        RPSChoice::Scissors => {
                            RPSChoice::Rock
                        }
                    }
                }
                MatchResult::Draw => {
                    match opponent {
                        RPSChoice::Rock => {
                            RPSChoice::Rock
                        },
                        RPSChoice::Paper => {
                            RPSChoice::Paper
                        }
                        RPSChoice::Scissors => {
                            RPSChoice::Scissors
                        }
                    }
                }
                MatchResult::OpponentWins => {
                    match opponent {
                        RPSChoice::Rock => {
                            RPSChoice::Scissors
                        },
                        RPSChoice::Paper => {
                            RPSChoice::Rock
                        }
                        RPSChoice::Scissors => {
                            RPSChoice::Paper
                        }
                    }
                }
            }
        }

    }
    pub fn evaluate_strategy_for_match_result(move_list : Vec<String>) -> u64{
        let mut sum = 0;
        for line in move_list{
            let mut characters = line.split(" ").collect::<Vec<&str>>();
            let (opponent,result) = (RPSChoice::from_symbol(characters[0]),MatchResult::from_symbol(characters[1]));
            let player = RPSChoice::get_move_for_match_result(&opponent, &result);
            sum +=  player.get_score() + result.get_score();
        }
        return sum;
    }
}