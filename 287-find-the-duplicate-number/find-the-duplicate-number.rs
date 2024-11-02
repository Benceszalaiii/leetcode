use std::collections::HashSet;
impl Solution {
    pub fn find_duplicate(nums: Vec<i32>) -> i32 {
    let mut v: HashSet<i32> = HashSet::new();
    let mut num = 0;
    for n in nums {
            if !v.insert(n){
                num = n;
                break;
            }
    }
    return num;
    }
}