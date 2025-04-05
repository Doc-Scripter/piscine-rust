use string_permutation_v2::*;

fn main() {
    let word = "thought";
    let word1 = "thougth";

    println!(
        "Is {:?} a permutation of {:?}? = {}",
        word,
        word1,
        is_permutation(word, word1)
    );
}

/*
Is "thought" a permutation of "thougth"? = true
*/