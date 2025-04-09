
#[derive(Debug, PartialEq)]
pub struct CipherError {
   pub expected: String
}

#[allow(unused)]
pub fn cipher(original: &str, ciphered: &str) -> Result<(), CipherError> {
    let mut res="".to_string();
    // res.in
    for (i,val) in original.to_owned().chars().enumerate(){
        let mirror_char = if val.is_ascii_lowercase() {
            // 'a' is 97, 'z' is 122
            // The formula: 'a' + 'z' - c
            // This maps 'a' to 'z', 'b' to 'y', etc.
            char::from_u32(219 - val as u32).unwrap()
        } else if val.is_ascii_uppercase() {
            // 'A' is 65, 'Z' is 90
            // The formula: 'A' + 'Z' - c
            char::from_u32(155 - val as u32).unwrap()
        } else {
            // Non-alphabetic characters remain unchanged
            val
        };
        res.push(mirror_char)
    }


//     let mut encrypt:String=original.to_string().split_whitespace().map(|e|e.chars().rev().collect::<String>()+" ").collect();
// // orig=encrypt.trim_end();
// println!("{orig}");
if ciphered !=res{
    return  Err(CipherError{expected:res})
}else{
    return Ok(())
}
}
/*
Ok(())
Err(CipherError { expected: "1Svool 2dliow!" })
*/