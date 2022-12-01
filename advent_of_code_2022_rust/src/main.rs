#[path = "common.rs"] mod common;
#[path = "day_1/part1_1.rs"] mod part1_1;
#[path = "day_1/part1_2.rs"] mod part1_2;

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

fn main() {
    let max_calory = part1_1::part1_1::find_max_calory(common::common::read_file_as_string(&"Input/input_day1.txt".to_string()));
    println!("max calory:{}",max_calory);
    let max_3_calory = part1_2::part1_2::find_max_3_calory(common::common::read_file_as_string(&"Input/input_day1.txt".to_string()));
    println!("max_3_calory:{}",max_3_calory);
}
