
use std::{collections::HashMap, hash::Hash};

/// Converts two slices into a HashMap where elements from the first slice are keys
/// and elements from the second slice are values.
///
/// # Arguments
/// * `a` - A slice containing elements to be used as keys
/// * `b` - A slice containing elements to be used as values
///
/// # Returns
/// A HashMap with references to elements from both slices
pub fn slices_to_map<'a, T:Eq+Hash, U>(a: &'a [T], b: &'a [U]) -> HashMap<&'a T, &'a U> {
   let mut res = HashMap::new();
    a.iter().zip(b.iter()).for_each(|(a, b)| {
        res.insert(a, b);
    });
    res

    
}