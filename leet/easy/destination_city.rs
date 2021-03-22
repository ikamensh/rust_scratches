// https://leetcode.com/problems/destination-city/

pub struct Solution {}

use std::collections::HashSet;

impl Solution {
    pub fn dest_city(paths: Vec<Vec<String>>) -> String {
        let mut origins = HashSet::new();
        let mut destinations = HashSet::new();

        for pair in paths {
            let mut it = pair.into_iter();
            origins.insert(it.next().unwrap());
            destinations.insert(it.next().unwrap());
        }

        let mut tmp = destinations.difference(&origins);
        tmp.next().expect("error!").to_string()
    }
}


#[cfg(test)]
mod tests {

    #[test]
    fn simple() {
        assert!(
            crate::Solution::dest_city(vec![vec!["a".to_string(), "b".to_string()]]) == "b"
        )
    }
}
