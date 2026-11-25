fn lower_bound_search(input: &Vec<u32>, target: u32) -> i32 {
    let mut left = 0;
    let mut right = input.len() - 1;
    while left < right {
        let mid = (left + right) / 2;
        if input[mid] > target {
            right = mid - 1;
        } else if input[mid] < target {
            left = mid + 1;
        } else {
            right = mid;
        }
    }
    if input[left] == target {
        return left as i32;
    }
    -1
}

fn upper_bound_search(input: &Vec<u32>, target: u32) -> i32 {
    let mut left = 0;
    let mut right = input.len() - 1;
    while left < right {
        let mid = (left + right) / 2 + 1;
        if input[mid] > target {
            right = mid - 1;
        } else if input[mid] < target {
            left = mid + 1;
        } else {
            left = mid;
        }
    }
    if input[right] == target {
        return right as i32;
    }
    -1
}
pub fn run(input: Vec<u32>, target: u32) -> (i32, i32) {
    (
        lower_bound_search(&input, target),
        upper_bound_search(&input, target),
    )
}

#[test]
fn test_first_and_last_occurances() {
    let input = vec![1, 2, 3, 4, 4, 4, 5, 6, 7, 8, 9, 10, 11];
    assert_eq!(run(input, 4), (3, 5));
}
