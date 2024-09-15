impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut numlist: Vec<(i32, usize)> = nums.iter().copied().enumerate().map(|(i, num)| (num, i)).collect();;
        numlist.sort_by_key(|&(a, _)| a);
        let mut left = 0;
        let mut right = numlist.len() - 1;
        loop{
            if left == right{
                break
            }
            if numlist[left].0 + numlist[right].0 == target{
                return vec![numlist[left].1 as i32, numlist[right].1 as i32]
            }
            if numlist[left].0 + numlist[right].0 > target{
                right -= 1;
            }
            else{
                left += 1;
            }
        }
        return vec![];
    }
}