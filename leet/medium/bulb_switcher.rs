//https://leetcode.com/problems/bulb-switcher-iv/submissions/

impl Solution {
    pub fn min_flips(target: String) -> i32 {
        let mut result = 0i32;
        let mut cur = '0';
        for c in target.chars() {
            if c != cur {
                cur = c;
                result += 1;
            }
        }
        result
    }
}
