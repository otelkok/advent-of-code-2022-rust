pub mod part4_1{
    pub struct CleaningRange{
        pub start : usize,
        pub end: usize,
    }
    impl CleaningRange{
        pub fn new(token: String) -> Self{
            let split : Vec<usize> = token.split("-").map(|boundary| boundary.parse::<usize>().unwrap()).collect();
            CleaningRange{start:split[0],end:split[1]}
        }
        pub fn does_contain(&self, to_contain:&CleaningRange) -> bool{
            (self.start <= to_contain.start && self.end >= to_contain.end) || (self.start >= to_contain.start && self.end <= to_contain.end)
        }
    }
    pub fn parse_input(input: Vec<String>) -> Vec<[CleaningRange;2]>{
        let mut result = Vec::new();
        for line in input{
            let split: Vec<String> = line.split(",").map(|split| split.to_string()).collect();    
            result.push([CleaningRange::new(split[0].clone()),CleaningRange::new(split[1].clone())]);
        }
        result
    }
    pub fn count_containing_pairs(pairs: Vec<[CleaningRange;2]>) -> u64{   
        let mut result = 0;
        for pair in pairs{
            result += match pair[0].does_contain(&pair[1]){
                    true => 1,
                    false => 0,
            }
        }
        result
    }
}