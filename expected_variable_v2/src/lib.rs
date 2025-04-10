fn edit_distance(s1: &str, s2: &str) -> usize {
    let s1: Vec<char> = s1.chars().collect();
    let s2: Vec<char> = s2.chars().collect();
    
    let m = s1.len();
    let n = s2.len();
    let mut dp = vec![vec![0; n + 1]; m + 1];

    // Initialize first row and column
    for i in 0..=m {
        dp[i][0] = i;
    }
    for j in 0..=n {
        dp[0][j] = j;
    }

    // Fill the dp table
    for i in 1..=m {
        for j in 1..=n {
            if s1[i-1] == s2[j-1] {
                dp[i][j] = dp[i-1][j-1];
            } else {
                dp[i][j] = dp[i-1][j-1]
                    .min(dp[i-1][j])
                    .min(dp[i][j-1]) + 1;
            }
        }
    }

    dp[m][n]
}

pub fn expected_variable(compare: &str, expected: &str) -> Option<String> {
    // Check if string is empty
    if compare.is_empty() || expected.is_empty() {
        return None;
    }

    // Stricter validation for snake_case
    let is_snake = compare.contains('_') && 
                   !compare.contains(' ') && 
                   !compare.contains("__") && // No double underscores
                   compare.chars().all(|c| c.is_alphanumeric() || c == '_') &&
                   !compare.starts_with('_') &&
                   !compare.ends_with('_');

    // Stricter validation for camelCase/PascalCase
    let is_camel = !compare.contains('_') && 
                   !compare.contains(' ') &&
                   compare.chars().all(|c| c.is_alphanumeric()) &&
                   !compare.chars().next().unwrap().is_numeric();
    
    // If neither snake_case nor camelCase/PascalCase, return None
    if !is_camel && !is_snake {
        return None;
    }

    // Calculate similarity
    let distance = edit_distance(&compare.to_lowercase(), &expected.to_lowercase());
    let max_len = expected.len().max(compare.len());
    let similarity = ((max_len - distance) as f64 / max_len as f64) * 100.0;
    
    if similarity > 50.0 {
        Some(format!("{:.0}%", similarity))
    } else {
        None
    }
}

