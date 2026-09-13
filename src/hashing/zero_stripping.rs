fn run(input: &mut Vec<Vec<i32>>) {
    let mut first_row_zero = false;
    let mut first_col_zero = false;

    for el in &mut input[0] {
        if *el == 0 {
            first_row_zero = true;
        }
    }
    for row in input.iter_mut() {
        let el = row[0];
        if el == 0 {
            first_col_zero = true;
        }
    }
    for i in 1..input.len()  {
        for j in 1..input[0].len() {
            let el = input[i][j];
            if el == 0 {
                input[i][0] = 0;
                input[0][j] = 0; 
            }
        }
    }

    for i in 1..input.len() {
        for j in 1..input[0].len() {
            let el = input[i][j];
            if input[0][j] == 0 || input[i][0] == 0 {
                input[i][j] = 0;
            }
        }
    }

    if first_row_zero {
        input[0].iter_mut().for_each(|el| *el= 0);
    }
    if first_col_zero {
        input.iter_mut().for_each(|row| row[0]= 0);
    }

}

mod test {
    use super::*;
    #[test]
    fn test_invalid(){
        let mut input = vec![
            vec![1,2,0,0,3,2,1,1,1],
            vec![1,2,0,0,3,2,1,1,1],
            vec![1,2,0,0,3,2,1,1,1],
            vec![1,2,0,0,3,2,1,1,1],
            vec![1,2,0,0,3,2,1,1,1],
            vec![1,2,0,0,3,2,1,1,1],
            vec![1,2,0,0,3,2,1,1,1],
            vec![1,2,0,0,3,2,1,1,1],
            vec![1,2,0,0,3,2,1,1,1]];

        let out = vec![
            vec![1,2,0,0,3,2,1,1,1],
            vec![1,2,0,0,3,2,1,1,1],
            vec![1,2,0,0,3,2,1,1,1],
            vec![1,2,0,0,3,2,1,1,1],
            vec![1,2,0,0,3,2,1,1,1],
            vec![1,2,0,0,3,2,1,1,1],
            vec![1,2,0,0,3,2,1,1,1],
            vec![1,2,0,0,3,2,1,1,1],
            vec![1,2,0,0,3,2,1,1,1]
            ];
        run(&mut input);
        assert_ne!(input, out);
    }

    #[test]
    fn test_valid() {

        let mut input = vec![
            vec![1,2,3,4,5],
            vec![6,0,8,9,10],
            vec![11,12,13,14,15],
            vec![16,17,18,19,0]
            ];

        let out = vec![
            vec![1,0,3,4,0],
            vec![0,0,0,0,0],
            vec![11,0,13,14,0],
            vec![0,0,0,0,0],
            ];
        run(&mut input);
        assert_eq!(input, out);
    }
}