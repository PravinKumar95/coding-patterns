use std::{cmp::max, collections::HashMap};

fn highest_freq(map: &HashMap<u8, u32>) -> usize {
    map.iter().fold(0_u32, |acc, x| {
        if x.1 > &acc {
            return *x.1;
        }
        acc
    }) as usize
}

pub fn run(input: &str, k: usize) -> usize {
    let mut map = HashMap::new();
    let mut left = 0;
    let mut right = 0;
    let mut win_len = (right - left + 1) as usize;
    map.insert(input.as_bytes()[0], 1);
    while right < input.len() {
        let diff = win_len - highest_freq(&map);
        if diff <= k {
            right += 1;
            if right >= input.len() {
                break;
            }
            let next_char = input.as_bytes()[right];
            let next_char_freq = *map.get(&next_char).unwrap_or(&0_u32);
            map.insert(next_char, next_char_freq + 1);
        } else {
            let curr_char = input.as_bytes()[left];
            let curr_char_freq = *map.get(&curr_char).unwrap_or(&0_u32);
            if curr_char_freq > 0 {
                map.insert(curr_char, curr_char_freq - 1);
            }
            left += 1;
            let next_char = input.as_bytes()[left];
            let next_char_freq = *map.get(&next_char).unwrap_or(&0_u32);
            map.insert(next_char, next_char_freq + 1);
        }
        win_len = (right - left + 1) as usize;
    }
    win_len
}

mod tests {
    use super::*;
    #[test]
    fn test_longest_substring_replacement() {
        let input = "aabcdcca";
        let k = 2;
        assert_eq!(run(input, k), 5);
    }
}
