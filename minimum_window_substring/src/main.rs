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
        let mut window_map: HashMap<char, usize>= HashMap::new();
        let mut result_map = t_map.clone();
        let mut first_sequence = 0;

        // window_map.insert(s.chars[0], 1);
        // for (i, c) in s.chars().enumerate() {

        for i in 0..s.len() {
            let c = s.chars().nth(i).unwrap();
            window_upper = i;

            if result_map.contains_key(&c) {
                let count = result_map.get_mut(&c).unwrap();
                if  *count > 0 {
                    *count -= 1;
                }
                if *count  == 0 {
                    result_map.remove(&c);
                }
            } else {
                if i != 0 && window_upper - 1 == window_lower && !result_map.is_empty() && result_map == t_map {
                    window_lower = i + 1;
                }
            }

            if result_map.is_empty() {
                if min_window >= window_upper - window_lower {
                    result_string = &s[window_lower..window_upper + 1];
                    min_window = window_upper - window_lower;
                }

                window_lower += 1;
                window_upper = window_lower;
                result_map = t_map.clone();


                window_lower = window_lower + 1;
                window_upper = i
            }
        }

        if min_window != usize::MAX {
            return String::from(result_string);
        }

        return String::from("");
    }
}

