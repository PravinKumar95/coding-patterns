pub fn run(inputs: Vec<i32>, target: i32) -> Option<(i32,i32)> {
    if inputs.len() < 2 {
        return None;
    }

    let mut left = 0;
    let mut right = inputs.len() - 1;
    while left < right {
        let sum = inputs[left] + inputs[right];
        if sum == target {
            return Some((left as i32, right as i32));
        }else if sum < target {
            left += 1;
        }else {
            right -= 1;
        }
    }
    return None;
}

mod tests {
    use std::assert_matches;
    use super::*;

    #[test]
    fn test_run() {
        assert_eq!(run(vec![],0), None);
        assert_eq!(run(vec![0],1), None);
        assert_eq!(run(vec![1,2],3), Some((0,1)));
        assert_matches!(run(vec![2,2,3],5), Some((0,2)) | Some((1,2)));
        assert_eq!(run(vec![-1,0,1],0), Some((0,2)));
        assert_eq!(run(vec![-2,-1,-1,0,2],2), Some((3,4)));
    }
}