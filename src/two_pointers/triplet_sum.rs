fn pair_sum(input: &Vec<i32>, target: i32, start: usize) -> Vec<(i32,i32)> {
    let mut result = Vec::new();
    let mut left = start;
    let mut right = input.len() - 1;
    while left < right {
        let a = input[left];
        let b = input[right];
        if left > start && input[left] == input[left - 1] {
            left += 1;
            continue;
        }
        let sum = a + b;
        if sum == target {
            result.push((left as i32, right as i32));
            left += 1;
        }else if sum < target {
            left += 1;
        }else {
            right -= 1;
        }
    }
    result
}

pub fn run(input: Vec<i32>) -> Vec<(i32,i32,i32)> {
    let mut result = Vec::new();
    if input.len() < 3 {
        return result;
    }
    let mut input = input;
    input.sort();
    let mut i = 0;
    while i < input.len() - 2 {
        let a = input[i];
        if i > 0 && input[i] == input[i - 1] {
            i += 1;
            continue;
        }
        let target = -a;
        let pairs = pair_sum(&input, target, i + 1);
        for (left, right) in pairs {
            result.push((i as i32, left, right));
        }
        i += 1;
    }
    result.iter().map(|&(i, j, k)| (input[i as usize], input[j as usize], input[k as usize])).collect()
}

mod tests {
    use std::assert_matches;
    use super::*;

    #[test]
    fn test_run() {
        assert_eq!(run(vec![]), vec![]);
        assert_eq!(run(vec![0]), vec![]);
        assert_eq!(run(vec![1,2]), vec![]);
        assert_eq!(run(vec![1,2,3]), vec![]);
        assert_eq!(run(vec![-1,0,1]), vec![(-1,0,1)]);
        assert_eq!(run(vec![-2,-1,-1,0,2]), vec![(-2,0,2), (-1,-1,2)]);
        assert_eq!(run(vec![0,0,1,-1,1,-1]), vec![(-1,0,1)]);
    }
}