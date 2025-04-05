use simple_hash_v2::*;

const SENTENCE: &str = "this is a very basic sentence with only a few repetitions. once again this is very basic but it should be enough for basic tests";

fn main() {
    let words = SENTENCE.split_ascii_whitespace().collect::<Vec<_>>();
    let frequency_count = word_frequency_counter(&words);

    println!("{:?}", frequency_count);
    println!("{}", nb_distinct_words(&frequency_count));
}
/*
{"tests": 1, "with": 1, "this": 2, "it": 1, "enough": 1, "is": 2, "but": 1, "sentence": 1, "only": 1, "basic": 3, "again": 1, "for": 1, "be": 1, "once": 1, "very": 2, "should": 1, "few": 1, "a": 2, "repetitions.": 1}
20
*/