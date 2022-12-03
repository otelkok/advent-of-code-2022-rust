pub mod part3_2{
    use crate::part3_1::part3_1::Rucksack;

    impl Rucksack{
        pub fn does_contain_item(&self, item: &char) -> bool{
            self.firstCompartment.does_contain_item(item) || self.secondCompartment.does_contain_item(item)
        }
    }
    pub fn find_badge_item(rucksacks: [&Rucksack;3]) -> char{
        for character in &rucksacks[0].firstCompartment.content{
            match is_common_item(rucksacks, character){
                true => return *character,
                false => {}
            }
        }
        for character in &rucksacks[0].secondCompartment.content{
            match is_common_item(rucksacks, character){
                true => return *character,
                false => {}
            }
        }
        for character in &rucksacks[1].firstCompartment.content{
            match is_common_item(rucksacks, character){
                true => return *character,
                false => {}
            }
        }
        for character in &rucksacks[1].secondCompartment.content{
            match is_common_item(rucksacks, character){
                true => return *character,
                false => {}
            }
        }
        for character in &rucksacks[2].firstCompartment.content{
            match is_common_item(rucksacks, character){
                true => return *character,
                false => {}
            }
        }
        for character in &rucksacks[2].secondCompartment.content{
            match is_common_item(rucksacks, character){
                true => return *character,
                false => {}
            }
        }
        panic!("no common items between:\n{:?}{:?}\n{:?}{:?}\n{:?}{:?}",rucksacks[0].firstCompartment.content,rucksacks[0].secondCompartment.content,
        rucksacks[1].firstCompartment.content,rucksacks[1].secondCompartment.content,
        rucksacks[2].firstCompartment.content,rucksacks[2].secondCompartment.content);
    }
    pub fn is_common_item(rucksacks: [&Rucksack;3], item : &char) -> bool{
        rucksacks[0].does_contain_item(item) && rucksacks[1].does_contain_item(item) && rucksacks[2].does_contain_item(item)

    }
}