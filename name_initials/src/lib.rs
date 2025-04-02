
pub fn initials(names: Vec<&str>) -> Vec<String> {
    let mut result = Vec::new();
    
    for name in names {
        let mut name_initials = String::new();
        
        for word in name.split_whitespace() {
            if let Some(first_char) = word.chars().next() {
                name_initials.push(first_char);
                name_initials.push_str(". ")

            }
        }
        
        if !name_initials.is_empty() {
            result.push(name_initials.trim().to_string());
        } else {
            result.push(String::new());
        }
    }
    
    result
}
