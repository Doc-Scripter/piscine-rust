pub fn arrange_phrase(phrase: &str) -> String {
    // Split the phrase into words
    let mut words: Vec<&str> = phrase.split_whitespace().collect();
    
    // Sort words based on the numeric value in each word
    words.sort_by(|a, b| {
        let a_digit = extract_number(a);
        let b_digit = extract_number(b);
        
        // Primary sort by whether the word has a digit
        match (a_digit, b_digit) {
            (Some(a_num), Some(b_num)) => a_num.cmp(&b_num), // If both have digits, sort by the numeric value
            (Some(_), None) => std::cmp::Ordering::Less,     // Words with digits come first
            (None, Some(_)) => std::cmp::Ordering::Greater,  // Words with digits come first
            (None, None) => std::cmp::Ordering::Equal,       // If neither has digits, maintain original order
        }
    });
    
    // Remove digits from each word and join them
    let result = words
        .iter()
        .map(|word| remove_digits(word))
        .collect::<Vec<String>>()
        .join(" ");
    
    result
}

// Helper function to extract the first number from a string
fn extract_number(s: &str) -> Option<u32> {
    let digits: String = s.chars().filter(|c| c.is_digit(10)).collect();
    if digits.is_empty() {
        None
    } else {
        digits.parse::<u32>().ok()
    }
}

// Helper function to remove all digits from a string
fn remove_digits(s: &str) -> String {
    s.chars().filter(|c| !c.is_digit(10)).collect()
}
