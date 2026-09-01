use std::collections::HashMap;

pub fn run(input: Vec<i32>, target: i32) -> Option<(i32,i32)> {
    if input.len() < 2 {
        return None;
    }
    let mut hash_map:  HashMap<i32,i32> = HashMap::new();
    let mut i = 0;
    while i < input.len() {
        let a = input[i];
        let a_compl = target - a;
        if hash_map.contains_key(&a) {
            return Some((i as i32, *hash_map.get(&a).unwrap() as i32));
        }
        hash_map.insert(a_compl, i as i32);
        i+=1;
    }
    None
}

mod tests {
    use std::assert_matches;

use super::*;
    #[test]
    fn test() {
        assert_eq!(run(vec![],3), None);
        assert_eq!(run(vec![1],0), None);
        assert_matches!(run(vec![1,2],3),  Some((0,1)) | Some((1,0)));
        assert_matches!(run(vec![2,2,3],5), Some((0,2)) | Some((1,2)) | Some((2,1)));
        assert_matches!(run(vec![-1,0,1],0), Some((0,2)) | Some((2,0)));
        assert_matches!(run(vec![-2,-1,-1,0,2],2), Some((3,4)) | Some((4,3)));
    }
}
