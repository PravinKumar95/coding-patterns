pub fn run(input: String) -> bool {
    if input.is_empty() {
        return true;
    }
    
    let mut left = 0;
    let mut right = input.len() -1;
    while left < right {
        let a = input.chars().nth(left).unwrap();
        let b = input.chars().nth(right).unwrap();
        if !a.is_alphanumeric() {
            left+=1;
            continue;
        }
        if !b.is_alphanumeric() {
            right-=1;
            continue;
        }
        if a != b {
            return false;
        }
        left+=1;
        right-=1;
    }
    true
}

mod tests {
    use super::run;
    #[test]
    fn test() {
        assert_eq!(run("Hello".into()),false);
        assert_eq!(run("".into()), true);
        assert_eq!(run("byte".into()), false);
        assert_eq!(run("racecar".into()), true);
        assert_eq!(run("a".into()), true);
        assert_eq!(run("aa".into()), true);
        assert_eq!(run("ab".into()), false);
        assert_eq!(run(" I . ' (?)".into()), true);
        assert_eq!(run("12.02.2021".into()),true);
        assert_eq!(run("12.02.2023".into()), false);
        
    }
}