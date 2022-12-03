pub mod part3_1{
    pub struct Rucksack{
        pub firstCompartment: Compartment,
        pub secondCompartment: Compartment,
    }
    pub struct Compartment{
        pub content: Vec<char>,
    }
    impl Rucksack{
        pub fn new(content: String) -> Self{
            let (first_half,second_half) = content.split_at(content.len()/2);
            Rucksack{
                firstCompartment: Compartment::new(first_half.to_string().chars().collect()),
                secondCompartment: Compartment::new(second_half.to_string().chars().collect())
            }
        }
        pub fn mistaken_items(&self) -> Vec<char>{
            let mut result: Vec<char> = Vec::new();
            for item in &self.firstCompartment.content{
                match self.secondCompartment.does_contain_item(item){
                    true if !result.contains(&item) => result.push(*item),
                    _ => {},
                }
            }
            for item in &self.secondCompartment.content{
                match self.firstCompartment.does_contain_item(item){
                    true if !result.contains(&item) => result.push(*item),
                    _ => {},
                }
            }
            result
        }
    }
    impl Compartment{
        pub fn new(content: Vec<char>) -> Self{
            Compartment{
                content
            }
        }
        pub fn does_contain_item(&self, item: &char) -> bool{
            self.content.contains(item)
        }
    }
    pub fn priority(items: Vec<char>) -> u32{
        let mut sum = 0;
        for item in items{
            sum += match item{
                character if character.is_uppercase() => {
                    item as u32 - 'A' as u32 + 27
                },
                character if character.is_lowercase() => {
                    item as u32 - 'a' as u32 + 1
                }
                _ => 0,
            };
        }
        sum
    }
}