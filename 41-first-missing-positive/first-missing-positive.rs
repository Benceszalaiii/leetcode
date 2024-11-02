impl Solution {
    pub fn first_missing_positive(nums: Vec<i32>) -> i32 {
        let mut v: Vec<i32> = nums.into_iter().filter(|x| *x > 0).collect();
        v.sort();
        let mut smallest = 1;
        for n in v{
            if smallest == n{
                smallest += 1;
            }
        }
        return smallest

    }
}
