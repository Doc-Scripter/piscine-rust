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
    
    // Iterate through each word
    for (i, word) in input.split_whitespace().enumerate() {
        if i > 0 {
            // Add a space before each word except the first one
            result.push(' ');
        }
        
        if let Some(first_char) = word.chars().next() {
            // Capitalize the first character
            result.push_str(&first_char.to_uppercase().to_string());
            
            // Add the rest of the word
            let rest = &word[first_char.len_utf8()..];
            result.push_str(rest);
        }
    }
    
    result.replace("\t","")
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
