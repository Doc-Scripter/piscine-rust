pub fn capitalize_first(input: &str) -> String {
    if input.is_empty(){
        return input.to_string()
    }
    let mut chars = input.chars();
    
    // Get the first character and convert to uppercase
    // The next() method returns an Option<char>
    let first_char = chars.next().unwrap().to_uppercase();
    
    // Create a new string with the uppercase first character
    // followed by the rest of the characters
    let rest_of_string = chars.as_str();
    
    // Combine the uppercase first character with the rest of the string
    format!("{}{}", first_char, rest_of_string)
}

pub fn title_case(input: &str) -> String {
    let mut result = String::new();
    let mut words = input.split_whitespace().peekable();
    
    // Track the position in the original string
    let mut pos = 0;
    
    while let Some(word) = words.next() {
        // Find the position of this word in the original string
        let word_pos = input[pos..].find(word).unwrap() + pos;
        
        // Add any whitespace characters (including tabs) that come before this word
        result.push_str(&input[pos..word_pos]);
        
        // Update position to after this word
        pos = word_pos + word.len();
        
        // Capitalize the first character of the word
        if let Some(first_char) = word.chars().next() {
            result.push_str(&first_char.to_uppercase().to_string());
            
            // Add the rest of the word
            let rest = &word[first_char.len_utf8()..];
            result.push_str(rest);
        }
    }
    
    // Add any remaining whitespace at the end
    if pos < input.len() {
        result.push_str(&input[pos..]);
    }
    
    result
}

pub fn change_case(input: &str) -> String {
    let mut result = String::with_capacity(input.len());
    
    for c in input.chars() {
        if c.is_uppercase() {
            // Convert uppercase to lowercase
            result.push_str(&c.to_lowercase().to_string());
        } else {
            // Convert lowercase to uppercase
            result.push_str(&c.to_uppercase().to_string());
        }
    }
    
    result
}
