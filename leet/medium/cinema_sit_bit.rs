//https://leetcode.com/problems/cinema-seat-allocation

use std::usize;
use std::collections::HashMap;

struct Solution {}

impl Solution {
    pub fn max_number_of_families(n: i32, reserved_seats: Vec<Vec<i32>>) -> i32 {

        let mut options: HashMap<i32, u8> = HashMap::new();

        for seat in reserved_seats {
            let row = seat[0];
            let seat_nr = seat[1];
            let entry = options.entry(row).or_insert(7);
            match seat_nr {
                1 => {},
                2 | 3 => {*entry &= 0b1111_1110},
                4 | 5 => {*entry &= 0b1111_1100},
                6 | 7 => {*entry &= 0b1111_1001},
                8 | 9 => {*entry &= 0b1111_1011},
                10 => {},
                _ => {println!("Unexpected seat nr {}", seat_nr); panic!()}
            }
        }

        let mut result = 0;
        for o in options.values(){
            match o{
                0 => {},
                8..=255 => {panic!()},
                5 | 7 => {result += 2}
                _ => {result += 1}
            }
        }
        assert!(options.len() as i32 <= n );
        result += 2 * (n - options.len() as i32);
        result
    }
}
