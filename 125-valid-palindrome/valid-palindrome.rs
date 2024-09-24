impl Solution {

pub fn is_palindrome(s: String) -> bool {
    let idx_last: usize;
    let a: String = s.chars().filter(|c: &char| c.is_ascii_alphanumeric()).map(|c: char| c.to_ascii_lowercase()).collect();
    let v: Vec<u8> = Vec::from(a.clone());
    let s_len: usize = v.len();
    if v.is_empty() || v.len() == 1 {
        return true;
    }
    if s_len % 2 == 0 {
        idx_last = s_len.clone() / 2;
    } else {
        idx_last = (s_len.clone() -1) / 2;
    }
    for idx in 0..(idx_last) {
        if v[idx] == v[s_len - idx - 1]{
            continue;
        }
        else{
        return false;
        }
    }
    return true;
}
}