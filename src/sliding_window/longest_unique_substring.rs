use std::{cmp::max, collections::HashSet};

pub fn run(input: &str) -> usize {
    let mut count = 0;
    let mut left = 0;
    let mut right = 0;
    let mut hash_set: HashSet<u32> = HashSet::new();
    while right < input.len() {
        while hash_set.contains(&(input.as_bytes()[right] as u32)) {
            hash_set.remove(&(input.as_bytes()[left] as u32));
            left += 1;
        }
        count = max(count, right - left + 1);
        hash_set.insert(input.as_bytes()[right] as u32);
        right += 1;
    }
    count
}

mod test {
    use super::*;
    #[test]
    fn test_unique_substring_true() {
        let input = "caabab";
        assert_eq!(run(input), 2);
    }
    #[test]
    fn test_unique_substring_true_2() {
        let input = "abcba";
        assert_eq!(run(input), 3);
    }
}
