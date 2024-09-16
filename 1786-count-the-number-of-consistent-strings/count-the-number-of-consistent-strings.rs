impl Solution {
    pub fn count_consistent_strings(allowed: String, words: Vec<String>) -> i32 {
    let av: Vec<char> = allowed.chars().collect();
    let mut counter: i32 = 0;
    for word in words {
        word.chars().all(|f| av.contains(&f) ).then(|| {counter += 1});
    }
    counter
    }
}