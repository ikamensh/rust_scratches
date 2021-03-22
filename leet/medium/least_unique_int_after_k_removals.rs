// https://leetcode.com/problems/least-number-of-unique-integers-after-k-removals/

use std::usize;
use std::collections::HashMap;

struct Solution {}

impl Solution {
    pub fn find_least_num_of_unique_ints(arr: Vec<i32>, k: i32) -> i32 {

        let mut counter: HashMap<i32, u32> = HashMap::new();

        for n in arr {
            let entry = counter.entry(n).or_insert(0);
            *entry += 1;
        }

        let mut counts: Vec<u32> = counter.values().cloned().collect();
        counts.sort();
        let counts = counts;
        let mut result = counts.len() as i32;
        let mut k_remaining = k as u32;
        for c in counts {
            if c > k_remaining { return result; }
            k_remaining -= c;
        }
        result
    }
}
