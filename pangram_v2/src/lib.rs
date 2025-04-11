pub fn is_pangram(s: &str) -> bool {
    if s.is_empty() {
        return false;
    }
    for v in 'a'..='z' {
        if !s.contains(v) {
            return false;
        }
    }
    true
}
