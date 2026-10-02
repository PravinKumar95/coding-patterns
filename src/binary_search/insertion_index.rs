fn run(input: &Vec<u32>, target: u32) -> usize {
    let mut left = 0;
    let mut right = input.len() - 1;
    let mut mid = 0;
    while left < right {
        mid = left + (right - left) / 2;
        if input[mid] < target {
            left = mid + 1;
        } else {
            right = mid;
        }
    }
    left
}

mod tests {
    use super::*;
    #[test]
    fn test_insertion_index() {
        let input = vec![1, 2, 4, 5, 7, 8, 9];
        assert_eq!(run(&input, 4), 2);
        assert_eq!(run(&input, 6), 4);
    }
}
