pub fn talking(text: &str) -> &str {
    if text == "" || text.is_empty() {
        return "Just say something!";
    }

    if text.chars().last() == Some('?') {
        if !text.chars().any(|x| x.is_ascii_lowercase()) {
            return "Quiet, I am thinking!";
        }
            return "Sure.";
    } else {
        if !text.chars().any(|x| x.is_ascii_lowercase()) {
            return "There is no need to yell, calm down!";
        }
        return "Interesting";
    }

}
