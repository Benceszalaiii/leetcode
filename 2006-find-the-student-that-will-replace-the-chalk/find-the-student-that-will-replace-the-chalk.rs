impl Solution {
    pub fn chalk_replacer(chalk: Vec<i32>, k: i32) -> i32 {
        let mut i = k.clone();
        let chalk_sum: i32 = chalk.clone().iter().sum();
        let mut idx = 0;
        loop {
            if i < chalk[idx] as i32{
                return idx as i32;
            }
            i -= chalk[idx];
                if idx >= chalk.len() -1{
                    idx = 0;
                }
                else{
                idx += 1;
                }
            
        }
    }
}