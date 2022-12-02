pub mod part1_2{
    pub fn find_max_3_calory(lines : Vec<String>) -> u64{
        let mut sum  = 0;
        let mut max_calory = u64::MIN;
        let mut second_max_calory = u64::MIN;
        let mut third_max_calory = u64::MIN;
        for line in lines{
            sum = match line {
                empty if empty.is_empty() => {
                    if sum > max_calory {
                        third_max_calory = second_max_calory;
                        second_max_calory = max_calory;
                        max_calory = sum;
                    }
                    0
                },
                val => sum + val.parse::<u64>().unwrap(),
            }
        }
        return max_calory + second_max_calory + third_max_calory;
    }
}