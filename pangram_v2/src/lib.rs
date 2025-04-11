pub fn is_pangram(s: &str) -> bool {
    if s.is_empty(){
        return  false;
    }
    for v in 'a'..'z'{
        if s.chars().any(|x|v ==x.to_ascii_lowercase()){
            continue;
        }else{
            return false;
        }
     
    }
    true

}