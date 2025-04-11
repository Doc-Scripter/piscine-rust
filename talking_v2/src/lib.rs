pub fn talking(text: &str) -> &str {
    if text.is_empty(){
        return  "Just say something!"
    }
    let mut yell = true;
    let mut question_yell = true;

    if text.chars().last() == Some('?') {
        
        if text.chars().any(|x| x.is_ascii_lowercase()) {
            question_yell = false;
        }
        if question_yell {
    
            return "Quiet, I am thinking!";
        }else{
            return "Sure.";
        }
    } else {
        if text.chars().any(|x| x.is_ascii_lowercase()) {
            yell = false;
        }
        if yell {
            return "There is no need to yell, calm down!";
        }
    }


    return "interesting";
}
