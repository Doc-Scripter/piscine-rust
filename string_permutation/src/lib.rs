use std::collections::HashMap;
pub fn is_permutation(s1: &str, s2: &str) -> bool {
    let mut s1_map: HashMap<char, bool> = HashMap::new();
    for i in s1.chars() {
        s1_map.insert(i, false);
    }
    for i in s2.chars() {
        if !s1_map.contains_key(&i) {
            return false
        }
    }
    true
}
