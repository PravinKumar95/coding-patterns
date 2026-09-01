use std::{cmp::min, ops::Mul};

pub fn run(input: Vec<i32>) -> i32 {
    let mut area = 0;
    if input.len() < 2 {
        return area;
    }
    let mut left = 0;
    let mut right = input.len() -1;
    while left < right {
        let a = input[left];
        let b = input[right];
        let curr_area = min(a, b).mul(right as i32 - left as i32);
        if curr_area > area {
            area = curr_area;
        }
        if a > b {
            right-=1;
        }else if b > a {
            left+=1;
        }else{
            left+=1;
            right-=1;
        }
    }
    area
}

mod tests {
    use super::*;
    #[test]
    fn test() {
        assert_eq!(run(vec![]),0);
        assert_eq!(run(vec![1]), 0);
        assert_eq!(run(vec![0, 1, 0]),0);
        assert_eq!(run(vec![3,3,3,3]), 9);
        assert_eq!(run(vec![1,2,3]),2);
        assert_eq!(run(vec![3,2,1]),2)
    }
}

