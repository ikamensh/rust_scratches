// https://leetcode.com/problems/reverse-words-in-a-string-iii

pub struct Solution {}

fn reverse_word(w: &str) -> String{
    let mut result = String::with_capacity(w.len());
    for c in w.chars().rev() {
        result.push(c);
    }
    result
}

impl Solution {
    pub fn reverse_words(s: String) -> String {
        let words = s.split(" ");
        let rev_words = words.into_iter().map(reverse_word);
        let v: Vec<String> = rev_words.collect();
        v.join(" ")
    }
}
