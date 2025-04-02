pub fn first_subword(mut s: String) -> String {
    let mut end_index = 1;
    let chars: Vec<char> = s.chars().collect();
    
    while end_index < chars.len() {
        let prev_char = chars[end_index - 1];
        let current_char = chars[end_index];
        
        if prev_char.is_lowercase() && current_char.is_uppercase() {
            break;
        }
        
        if !current_char.is_alphabetic() {
            break;
        }
        
        end_index += 1;
    }
    
    s.truncate(end_index);
    s
}