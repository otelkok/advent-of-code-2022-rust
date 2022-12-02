pub mod part2_1{
    pub enum RPSChoice{
        Rock,
        Paper,
        Scissors,
    }
    pub enum MatchResult{
        PlayerWins,
        Draw,
        OpponentWins,
    }
    impl MatchResult{
        pub fn from_symbol(symbol: &str) -> MatchResult{
            match symbol{
                "X" => MatchResult::OpponentWins,
                "Y" => MatchResult::Draw,
                _ => MatchResult::PlayerWins,
            }
        }
        pub fn get_score(&self) -> u64{
            match self{
                MatchResult::PlayerWins => 6,
                MatchResult::Draw => 3,
                MatchResult::OpponentWins =>0
            }
        }
    }
    impl RPSChoice{
        pub fn get_score(&self) -> u64{
            match self{
                RPSChoice::Rock => 1,
                RPSChoice::Paper => 2,
                RPSChoice::Scissors => 3,
            }
        }
        pub fn get_match_score(opponent : &RPSChoice, player : &RPSChoice) -> u64{
            let mut match_score = match RPSChoice::match_result(opponent,player){
                MatchResult::PlayerWins => 6,
                MatchResult::Draw => 3,
                MatchResult::OpponentWins => 0,
            };
            match_score += match player {
                RPSChoice::Rock => 1,
                RPSChoice::Paper => 2,
                RPSChoice::Scissors => 3,
            };
            match_score
        }
        fn match_result(opponent: &RPSChoice, player: &RPSChoice) -> MatchResult{
            match player{
                RPSChoice::Rock => {
                    match opponent{
                        RPSChoice::Rock => MatchResult::Draw,
                        RPSChoice::Paper => MatchResult::OpponentWins,
                        RPSChoice::Scissors => MatchResult::PlayerWins,
                    }
                },
                RPSChoice::Paper => {
                    match opponent{
                        RPSChoice::Rock => MatchResult::PlayerWins,
                        RPSChoice::Paper => MatchResult::Draw,
                        RPSChoice::Scissors => MatchResult::OpponentWins,
                    }
                }
                RPSChoice::Scissors => {
                    match opponent {
                        RPSChoice::Rock => MatchResult::OpponentWins,
                        RPSChoice::Paper => MatchResult::PlayerWins,
                        RPSChoice::Scissors => MatchResult::Draw,
                    }
                }
            }
        }
        pub fn from_symbol(symbol : &str) -> RPSChoice{
            match symbol{
                "X" | "A" => RPSChoice::Rock,
                "Y" | "B" => RPSChoice::Paper,
                _ => RPSChoice::Scissors,
            }
        }
    }
    
    pub fn evaluate_strategy(move_list : Vec<String>) -> u64{
        let mut sum = 0;
        for line in move_list{
            let mut characters = line.split(" ").collect::<Vec<&str>>();
            let (opponent,player) = (RPSChoice::from_symbol(characters[0]),RPSChoice::from_symbol(characters[1]));
            sum += RPSChoice::get_match_score(&opponent,&player);
        }
        return sum;
    }
}
