#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn it_handles_no_solution() {
        assert_eq!(Solution::min_window(String::from("a"), String::from("b")), "");
        assert_eq!(Solution::min_window(String::from("aaaaaaaaaaaaa"), String::from("b")), "");
    }

    #[test]
    fn it_handles_exact_match() {
        assert_eq!(Solution::min_window(String::from("A"), String::from("A")), "A");
        assert_eq!(Solution::min_window(String::from("ABC"), String::from("ABC")), "ABC");
    }

    #[test]
    fn it_handles_reverse() {
        assert_eq!(Solution::min_window(String::from("CBA"), String::from("ABC")), "CBA");
    }

    #[test]
    fn it_handles_multiple_solutions() {
        assert_eq!(Solution::min_window(String::from("BADEEAZZCBD"), String::from("BCD")), "CBD");
    }

    #[test]
    fn it_handles_general() {
        assert_eq!(Solution::min_window(String::from("ADOBECODEBANC"), String::from("ABC")), "BANC");
    }
}