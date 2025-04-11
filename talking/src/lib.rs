pub fn talking(text: &str) -> &str {
    // Check for empty string first
    if text.is_empty() {
        return "Just say something!";
    }

    // Check if it's a question (ends with '?')
    if text.chars().last() == Some('?') {
        // For questions, we need to check if it contains any alphabetic characters
        // and if all alphabetic characters are uppercase
        let has_letters = text.chars().any(|c| c.is_alphabetic());
        let all_uppercase = text.chars()
            .filter(|c| c.is_alphabetic())
            .all(|c| c.is_uppercase());
        
        if has_letters && all_uppercase {
            return "Quiet, I am thinking!";
        } else {
            return "Sure.";
        }
    } else {
        // For non-questions, check if it's yelling (all uppercase with at least one letter)
        let has_letters = text.chars().any(|c| c.is_alphabetic());
        let all_uppercase = text.chars()
            .filter(|c| c.is_alphabetic())
            .all(|c| c.is_uppercase());
        
        if has_letters && all_uppercase {
            return "There is no need to yell, calm down!";
        } else {
            return "Interesting";
        }
    }
}