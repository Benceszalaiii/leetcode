impl Solution {
    pub fn valid_palindrome(s: String) -> bool {
        is_valid_palindrome::<false>(s.as_bytes())
    }
}

fn is_valid_palindrome<const SKIPPED: bool>(mut s: &[u8])->bool{
    loop {
        match s {
            [b, middle @ .., e] if b == e => s = middle,
            [_, .., _] if !SKIPPED => return is_valid_palindrome::<true>(&s[1..])
                || is_valid_palindrome::<true>(s.split_last().unwrap().1),
            [_, .., _] if SKIPPED => return false,
            _ => return true,
        }
    }
}