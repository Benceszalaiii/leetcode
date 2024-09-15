use std::collections::HashSet;

impl Solution {
    pub fn contains_duplicate(nums: Vec<i32>) -> bool {
        let numhash: HashSet<i32> = HashSet::from_iter(nums.clone());
        return numhash.len() != nums.len();
    }
}