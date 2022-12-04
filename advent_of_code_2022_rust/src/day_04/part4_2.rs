#[path = "./part4_1.rs"] mod part4_1;
use part4_1::part4_1::CleaningRange;
pub mod part4_2{
    use crate::part4_1::part4_1::CleaningRange;

    impl CleaningRange{
        pub fn does_overlap(&self, to_overlap: &CleaningRange) -> bool{
            println!("Overlapping for pairs:{}-{},{}-{}:{}",self.start,self.end,to_overlap.start,to_overlap.end, (self.start <= to_overlap.start && self.end >= to_overlap.start) || (to_overlap.start <= self.start && to_overlap.end >= self.start));
            (self.start <= to_overlap.start && self.end >= to_overlap.start) || (to_overlap.start <= self.start && to_overlap.end >= self.start)
        }
    }
    pub fn count_overlapping_pairs(pairs: Vec<[CleaningRange;2]>) -> u64{
        let mut result = 0;
        for pair in pairs{
            result += match pair[0].does_overlap(&pair[1]) {
                true =>1,
                false =>0,
            }
        }
        result
    }
}