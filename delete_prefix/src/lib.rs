pub fn delete_prefix<'a>(prefix: &str, s: &'a str) -> Option<&'a str> {
    if s.starts_with(prefix){
        return  Some(&s[prefix.len()..s.len()]);
    }else{
        None
    }
}