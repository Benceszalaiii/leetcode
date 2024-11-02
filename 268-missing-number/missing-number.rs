impl Solution {
    pub fn missing_number(nums: Vec<i32>) -> i32 {
                let mut v: Vec<i32> = nums.into_iter().filter(|x| *x >= 0).collect();
        v.sort();
        let mut smallest = 0;
        for n in v{
            if smallest == n{
                smallest += 1;
            }
        }
        return smallest
    }
}