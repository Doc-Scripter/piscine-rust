pub fn arrange_phrase(phrase: &str) -> String {
    // Count the number of words to pre-allocate the vector
    let word_count = phrase.split_whitespace().count();
    
    // Create a vector of (word, number) pairs to avoid repeated extraction
    let mut word_pairs: Vec<(&str, Option<u32>)> = Vec::with_capacity(word_count);
    
    for word in phrase.split_whitespace() {
        word_pairs.push((word, extract_number(word)));
    }
    
    // Sort the pairs based on the numeric value
    word_pairs.sort_by(|a, b| {
        match (a.1, b.1) {
            (Some(a_num), Some(b_num)) => a_num.cmp(&b_num),
            (Some(_), None) => std::cmp::Ordering::Less,
            (None, Some(_)) => std::cmp::Ordering::Greater,
            (None, None) => std::cmp::Ordering::Equal,
        }
    });
    
    // Pre-calculate the length of the result string to avoid reallocations
    let result_len = word_pairs.iter()
        .map(|(word, _)| {
            // Length of word without digits + 1 for space
            word.chars().filter(|c| !c.is_digit(10)).count() + 1
        })
        .sum::<usize>()
        .saturating_sub(1); // Subtract 1 because there's no space after the last word
    
    // Build the result string with pre-allocated capacity
    let mut result = String::with_capacity(result_len);
    
    for (i, (word, _)) in word_pairs.iter().enumerate() {
        // Add each character that is not a digit
        for c in word.chars() {
            if !c.is_digit(10) {
                result.push(c);
            }
        }
        
        // Add space between words, but not after the last word
        if i < word_pairs.len() - 1 {
            result.push(' ');
        }
    }
    
    result
}

// Helper function to extract the first number from a string
fn extract_number(s: &str) -> Option<u32> {
    let mut number = 0;
    let mut has_digit = false;
    
    for c in s.chars() {
        if c.is_digit(10) {
            has_digit = true;
            if let Some(digit) = c.to_digit(10) {
                number = number * 10 + digit;
            }
        }
    }
    
    if has_digit {
        Some(number)
    } else {
        None
    }
}
