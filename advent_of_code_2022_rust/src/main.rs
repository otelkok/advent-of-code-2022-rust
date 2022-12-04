#[path = "common.rs"] mod common;
#[path = "day_01/part1_1.rs"] mod part1_1;
#[path = "day_01/part1_2.rs"] mod part1_2;
#[path = "day_02/part2_1.rs"] mod part2_1;
#[path = "day_02/part2_2.rs"] mod part2_2;
#[path = "day_03/part3_1.rs"] mod part3_1;
#[path = "day_03/part3_2.rs"] mod part3_2;
#[path = "day_04/part4_1.rs"] mod part4_1;
#[path = "day_04/part4_2.rs"] mod part4_2;

#[test]
fn control_day1_a(){
    let max_calory = part1_1::part1_1::find_max_calory(common::common::read_file_as_string(&"Input/input_day1.txt".to_string()));
    assert_eq!(max_calory,72478);
}
#[test]
fn control_day1_b(){
    let max_3_calory = part1_2::part1_2::find_max_3_calory(common::common::read_file_as_string(&"Input/input_day1.txt".to_string()));
    assert_eq!(max_3_calory,210367);
}
#[test]
fn control_day2_a(){
    let score = part2_1::part2_1::evaluate_strategy(common::common::read_file_as_string(&"Input/input_day2.txt".to_string()));
    assert_eq!(score,11666);
}
#[test]
fn control_day2_b(){
    let score = part2_2::part2_2::evaluate_strategy_for_match_result(common::common::read_file_as_string(&"Input/input_day2.txt".to_string()));
    assert_eq!(score,12767);
}
#[test]
fn control_day3_a(){
    let lines = common::common::read_file_as_string(&"Input/input_day3.txt".to_string());
    let mut rucksacks: Vec<part3_1::part3_1::Rucksack> = Vec::new();
    for line in lines{
        rucksacks.push(part3_1::part3_1::Rucksack::new(line));
    }
    let mut priority_sum = 0;
    for compartment in rucksacks{
        priority_sum += part3_1::part3_1::priority(compartment.mistaken_items());
    }
    assert_eq!(priority_sum,7742);
}
#[test]
fn control_day3_b(){
    let lines = common::common::read_file_as_string(&"Input/input_day3.txt".to_string());
    let mut rucksacks: Vec<part3_1::part3_1::Rucksack> = Vec::new();
    for line in lines{
        rucksacks.push(part3_1::part3_1::Rucksack::new(line));
    }
    let mut badges = Vec::new();
    for index in (2..rucksacks.len()).step_by(3){
        badges.push(part3_2::part3_2::find_badge_item([&rucksacks[index-2],&rucksacks[index-1],&rucksacks[index]]));
    }
    let sum = part3_1::part3_1::priority(badges);
    assert_eq!(sum,2276);
}
#[test]
fn control_day4_a(){
    let lines = common::common::read_file_as_string(&"Input/input_day4.txt".to_string());
    let count = part4_1::part4_1::count_containing_pairs(part4_1::part4_1::parse_input(lines));
    assert_eq!(count,530);
}
#[test]
fn control_day4_b(){
    let lines = common::common::read_file_as_string(&"Input/input_day4.txt".to_string());
    let count = part4_2::part4_2::count_overlapping_pairs(part4_1::part4_1::parse_input(lines));
    assert_eq!(count,903);
}
fn main() {
    let lines = common::common::read_file_as_string(&"Input/input_day4.txt".to_string());
    let count = part4_2::part4_2::count_overlapping_pairs(part4_1::part4_1::parse_input(lines));
    println!("{}",count);
}
