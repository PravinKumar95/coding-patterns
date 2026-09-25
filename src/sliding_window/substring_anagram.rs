fn is_equal(a: &[u32; 26], b: &[u32; 26]) -> bool {
    let mut out = true;
    for i in 0..26 {
        let x = a[i];
        let y = b[i];
        if x != y {
            out = false;
        }
    }
    out
}
fn move_right(left: &mut usize, right: &mut usize, window_freq: &mut [u32; 26], input: &str) {
    let left_idx = (input.as_bytes()[*left] as u8 - 'a' as u8) as usize;
    if window_freq[left_idx] > 0 {
        window_freq[left_idx] -= 1;
    }
    *left += 1;
    *right += 1;
    if right >= &mut input.len() {
        return;
    }
    let right_idx = (input.as_bytes()[*right] as u8 - 'a' as u8) as usize;
    window_freq[right_idx] += 1;
}
pub fn run(input: &str, sub: &str) -> u32 {
    let mut count = 0;
    let mut sub_freq = [0; 26];
    for c in sub.chars() {
        let idx = (c as u8 - 'a' as u8) as usize;
        sub_freq[idx] += 1;
    }
    let mut window_freq = [0; 26];
    let mut left = 0;
    for i in 0..sub.chars().count() {
        let c = input.as_bytes()[i];
        let idx = (c as u8 - 'a' as u8) as usize;
        window_freq[idx] += 1;
    }
    let mut right = sub.chars().count() - 1;
    while right < input.len() {
        if is_equal(&window_freq, &sub_freq) {
            count += 1;
        }
        move_right(&mut left, &mut right, &mut window_freq, input);
    }
    count
}

mod test {
    use super::*;
    #[test]
    fn test_anagram_true() {
        let input = "caabab";
        let sub = "aba";
        assert_eq!(run(input, sub), 2);
    }
}
