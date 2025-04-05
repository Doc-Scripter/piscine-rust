use std::collections::HashMap;
pub fn is_permutation(s1: &str, s2: &str) -> bool {
    let mut s2_map: HashMap<char, i32> = HashMap::new();
    let mut s1_map: HashMap<char, i32> = HashMap::new();

    for i in s2.chars() {
        *s2_map.entry( i).or_insert(0) += 1;
    }

    for i in s1.chars() {
        if !s2_map.contains_key(&i) {
            return false;
        }
    }

    for i in s1.chars() {
        *s1_map.entry(i).or_insert(0) += 1;
    }

    for (k, v) in s2_map {
        if *s1_map.get(&k).unwrap_or(&0) != v {
            return false;
        }
    }

    true
}
