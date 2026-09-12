use std::collections::HashMap;

fn run(input: &Vec<Vec<i32>>) -> bool {
    println!("input: {:?}", input);
    let mut row_maps = vec![];
    let mut col_maps = vec![];
    let mut subset_maps = vec![];

    for row in input {
        row_maps.push(HashMap::new());
        col_maps.push(HashMap::new());
        subset_maps.push(HashMap::new());
    }

    let row_len = row_maps.len();
    let col_len = col_maps.len();

    for i in 0..row_len {
        for j in 0..col_len {
            let el = input[i][j];
            if el == 0 {
                continue;
            }
            if row_maps[i].contains_key(&el) {
                return false
            }
            if col_maps[j].contains_key(&el) {
                return false;
            }
            if subset_maps[(i/3) * 3 + (j/3)] .contains_key(&el){
                return false;
            }
            row_maps[i].insert(el, 1);
            col_maps[i].insert(el,1);
            subset_maps[(i/3) * 3 + (j/3)].insert(el, 1);
        }
    }
    true
}

mod test {
    use super::*;
    #[test]
    fn test_invalid() {
        let input = vec![
            vec![1,2,3,4,5,6,7,8,9],
            vec![1,2,3,4,5,6,7,8,9],
            vec![1,2,3,4,5,6,7,8,9],
            vec![1,2,3,4,5,6,7,8,9],
            vec![1,2,3,4,5,6,7,8,9],
            vec![1,2,3,4,5,6,7,8,9],
            vec![1,2,3,4,5,6,7,8,9],
            vec![1,2,3,4,5,6,7,8,9],
            vec![1,2,3,4,5,6,7,8,9],
        ];
        assert_eq!(run(&input), false)
    }
        #[test]
    fn test_valid() {
        let input = vec![
            vec![3,0,6,0,5,8,4,0,0],
            vec![5,2,0,0,0,0,0,0,0],
            vec![0,8,7,0,0,0,0,3,1],
            vec![1,0,2,5,0,0,3,2,0],
            vec![9,0,0,8,6,3,0,0,5],
            vec![0,5,0,0,9,0,6,0,0],
            vec![0,3,0,0,0,8,2,5,0],
            vec![0,1,0,0,0,0,0,7,4],
            vec![0,0,5,2,0,6,0,0,0],
        ];
        assert_eq!(run(&input), false)
    }
}