fn square_sum(num: u32) -> u32 {
    let mut sum = 0;
    let mut val = num;
    let mut last_digit = 0;
    while val != 0 {
        last_digit = val % 10;
        sum += last_digit * last_digit;
        val = val / 10;
    }
    sum
}
fn run(input: u32) -> bool {
    let mut slow = square_sum(input);
    let mut fast = square_sum(square_sum(input));
    while slow != 1 || fast != 1 {
        if slow == fast {
            return false;
        }
        slow = square_sum(slow);
        fast = square_sum(square_sum(fast));
    }
    return true;
}

mod tests {
    use super::*;
    #[test]
    fn test_square_sum() {
        assert_eq!(square_sum(41), 17);
    }
    #[test]
    fn test_happy_number() {
        assert_eq!(run(23), true);
    }
    #[test]
    fn test_not_happy_number() {
        assert_eq!(run(116), false);
    }
}
