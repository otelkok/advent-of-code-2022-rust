#[path = "common.rs"] mod common;
#[path = "day_01/part1_1.rs"] mod part1_1;
#[path = "day_01/part1_2.rs"] mod part1_2;
#[path = "day_02/part2_1.rs"] mod part2_1;
#[path = "day_02/part2_2.rs"] mod part2_2;
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
fn main() {
    let score = part2_2::part2_2::evaluate_strategy_for_match_result(common::common::read_file_as_string(&"Input/input_day2.txt".to_string()));
    println!("Score:{}",score);
}
