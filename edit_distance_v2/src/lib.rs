pub fn edit_distance(source: &str, target: &str) -> usize {
    // Convert strings to vectors of characters for easier indexing
    let source_chars: Vec<char> = source.chars().collect();
    let target_chars: Vec<char> = target.chars().collect();
    
    let source_len = source_chars.len();
    let target_len = target_chars.len();
    
    // Handle edge cases
    if source_len == 0 {
        return target_len;
    }
    if target_len == 0 {
        return source_len;
    }
    
    // Create a matrix to store the edit distances
    // dp[i][j] represents the edit distance between source[0..i] and target[0..j]
    let mut dp = vec![vec![0; target_len + 1]; source_len + 1];
    
    // Initialize the first row and column
    for i in 0..=source_len {
        dp[i][0] = i;
    }
    for j in 0..=target_len {
        dp[0][j] = j;
    }
    
    // Fill the dp matrix
    for i in 1..=source_len {
        for j in 1..=target_len {
            // If characters match, no operation needed for this position
            if source_chars[i - 1] == target_chars[j - 1] {
                dp[i][j] = dp[i - 1][j - 1];
            } else {
                // Find the minimum cost operation (insertion, deletion, substitution)
                dp[i][j] = 1 + std::cmp::min(
                    dp[i - 1][j],     // deletion
                    std::cmp::min(
                        dp[i][j - 1], // insertion
                        dp[i - 1][j - 1] // substitution
                    )
                );
            }
        }
    }
    
    // The bottom-right cell contains the edit distance between the full strings
    dp[source_len][target_len]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_specific_case() {
        let source = "alignment";
        let target = "assignment";
        let result = edit_distance(source, target);
        assert_eq!(result, 4);
    }

    #[test]
    fn test_empty_strings() {
        assert_eq!(edit_distance("", ""), 0);
        assert_eq!(edit_distance("abc", ""), 3);
        assert_eq!(edit_distance("", "xyz"), 3);
    }

    #[test]
    fn test_identical_strings() {
        assert_eq!(edit_distance("same", "same"), 0);
    }

    #[test]
    fn test_simple_edits() {
        assert_eq!(edit_distance("kitten", "sitting"), 3);
        assert_eq!(edit_distance("saturday", "sunday"), 3);
    }
}
