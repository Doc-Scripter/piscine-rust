pub fn get_products(arr: Vec<usize>) -> Vec<usize> {
    if arr.is_empty() || arr.len() == 1 {
        return vec![];  // Return empty vector for both empty and single element arrays
    }
    
    arr.iter().enumerate().map(|(i, _)| {
        let mut prod = 1;
        for (j, &val) in arr.iter().enumerate() {
            if i != j {  // Compare indices instead of values
                prod *= val;
            }
        }
        prod
    }).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty_case() {
        assert_eq!(get_products(vec![]), vec![]);
    }

    #[test]
    fn test_single_case() {
        assert_eq!(get_products(vec![2]), vec![]);
    }

    #[test]
    fn test_multiple() {
        assert_eq!(get_products(vec![1, 2, 3, 4, 5]), vec![120, 60, 40, 30, 24]);
    }
}
