use std::collections::HashMap;
pub fn word_frequency_counter(words: Vec<&str>) -> HashMap<&str, usize> {
    let mut hash: HashMap<&str, usize> = HashMap::new();
    for val in words {
        if hash.contains_key(&val){
            hash.insert(val, hash.get(&val).unwrap() + 1);
        }else{
            hash.insert(val, 1);
        }
    }
    hash
}

pub fn nb_distinct_words(frequency_count: &HashMap<&str, usize>) -> usize {
    frequency_count.len()
}
/*
pub fn word_frequency_counter(words: Vec<&str>) -> HashMap<String, usize> {
    let mut frequency_map = HashMap::new();
    
    for word in words {
        let count = frequency_map.entry(word.to_string()).or_insert(0);
        *count += 1;
    }
    
    frequency_map
}

*/
