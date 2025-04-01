pub fn rev_str(input: &str) -> String {
    let mut chars = String::new();
    for letter in input.chars().rev(){
        chars.push(letter);
    }
    chars
}