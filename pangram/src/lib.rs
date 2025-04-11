pub fn is_pangram(s: &str) -> bool {
    for v in 'a'..'z'{
        if s.chars().any(|x|v ==x){
            continue;
        }else{
            return false;
        }
     
    }
    true

}