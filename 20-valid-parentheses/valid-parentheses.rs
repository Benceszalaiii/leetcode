impl Solution {
    pub fn is_valid(s: String) -> bool {
        fn swap_p(c: char) -> char{
            return match c {
                '(' => ')',
                ')' => '(',
                '{' => '}',
                '}' => '{',
                '[' => ']',
                ']' => '[',
                _ => char::default()
            }
        }
        let mut needs_closing: Vec<char> = vec![];
        let mut latest: char = char::default();
        for x in s.chars(){

            if vec!['(', '{', '['].contains(&x){
                latest = x.clone();
                needs_closing.push(x);
            }
            if vec![')', '}', ']'].contains(&x){
                if swap_p(latest.clone()) != x{
                    return false;
                }
                else{
                    needs_closing.pop();
                    if needs_closing.is_empty(){
                        latest = char::default();
                    }
                    else{
                    latest = needs_closing[needs_closing.len() - 1];
                    }
                }
            }
        }
        if needs_closing.is_empty(){
            return true;
        }
        return false;
    }
}