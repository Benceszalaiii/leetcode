impl Solution {
pub fn find_min_difference(time_points: Vec<String>) -> i32 {
        fn handle_input(s: String) -> i32 {
            let raw: Vec<&str> = s.split(":").collect();
            let mut res: Vec<i32> = Vec::new();
            for r in raw{
                res.push(r.parse::<i32>().expect("Error: Input invalid."))
            }
            
            return res.get(0).expect("Invalid input somehow.") * 60  + res.get(1).expect("Invalid input somehow");
        }
        let mut v: Vec<i32> = Vec::new();
        for time in time_points{
            v.push(handle_input(time));
        };
        v.sort();

        let mut best = 1440 - (v.last().expect("?") - v.first().expect("?"));
        for i in 1..v.len(){
            if v[i] - v[i-1] < best{
                best = v[i] - v[i-1];
            }
        };
        best

        
        }

}