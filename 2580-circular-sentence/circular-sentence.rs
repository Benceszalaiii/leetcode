impl Solution {
    pub fn is_circular_sentence(sentence: String) -> bool {
    let chars: Vec<char> = sentence.chars().collect();
    if chars.first() != chars.last() {
        return false;
    }

    chars
        .windows(3)
        .filter(|window| window[1] == ' ')
        .all(|window| window[0] == window[2])
    }
}