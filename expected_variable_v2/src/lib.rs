use case::CaseExt;

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
    // Check if string is in valid case format
    let is_snake = compare == compare.to_snake();
    // Also accept Pascal case by checking if string equals its camel case after capitalizing first letter
    let is_camel = compare.is_camel_lowercase() || 
                  compare == compare.to_camel() ||
                  compare == format!("{}{}", &compare[..1].to_uppercase(), &compare[1..]);
    
    if !is_camel && !is_snake {
        return None;
    }

    // Calculate edit distance
    let distance = edit_distance(&compare.to_lowercase(), &expected.to_lowercase());
    
    // Calculate similarity percentage
    let max_len = expected.len().max(compare.len());
    let similarity = ((max_len - distance) as f64 / max_len as f64) * 100.0;
    
    // Return percentage if similarity is greater than 50%
    if similarity > 50.0 {
        Some(format!("{:.0}%", similarity))
    } else {
        None
    }
}

