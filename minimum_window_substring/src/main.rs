mod tests;

use std::collections::HashMap;

fn main() {

}

struct Solution{}

impl Solution {
    pub fn min_window(s: String, t: String) -> String {
        if t.is_empty() {
            return String::new();
        }

        if t.len() > s.len() {
            return String::new();
        }

        if t == s {
            return s.clone();
        }

        // Use this for quick result look ups
        let mut t_map: HashMap<char, usize> = HashMap::new();
        for c in t.chars() {
            if t_map.contains_key(&c) {
                t_map.insert(c, t_map.get(&c).unwrap() + 1);
            } else {
                t_map.insert(c, 1);
            }
        }

        let mut window_lower: usize = 0;
        let mut window_upper: usize = 0;
        let mut min_window = usize::MAX;
        let mut result_string = "";
        let mut result_map = t_map.clone();


        while window_upper != s.len() && window_lower != s.len() {
            let c = s.chars().nth(window_upper).unwrap();

            if result_map.contains_key(&c) {
                let count = result_map.get_mut(&c).unwrap();
                if  *count > 0 {
                    *count -= 1;
                }
                if *count  == 0 {
                    result_map.remove(&c);
                }
            } else {
                if window_upper != 0 && window_upper - 1 == window_lower && !result_map.is_empty() && result_map == t_map {
                    window_lower += 1;
                }
            }

            if result_map.is_empty() {
                result_map = t_map.clone();

                if min_window > window_upper - window_lower {
                    result_string = &s[window_lower..window_upper + 1];
                    min_window = window_upper - window_lower;
                }
                window_lower += 1;
                let mut current = s.chars().nth(window_lower).unwrap();
                while window_lower < window_upper && !t.contains(current) {
                    window_lower += 1;
                    current = s.chars().nth(window_lower).unwrap();
                }
                let count = result_map.get_mut(&current).unwrap();
                if  *count > 0 {
                    *count -= 1;
                }
                if *count == 0 {
                    result_map.remove(&current);
                }
                window_upper = window_lower;
            }
            window_upper += 1;
        }

        if min_window != usize::MAX {
            return String::from(result_string);
        }

        return String::from("");
    }
}

