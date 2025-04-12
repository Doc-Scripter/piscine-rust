
pub fn pig_latin(text: &str) -> String {
    let mut orig =text.to_string();
    let mut iterate= orig.chars();
    // if text.to_string().starts_with("qu") {
    //     let mut changed_text:String = text.to_string().chars().rev().collect();
    //     changed_text.truncate(changed_text.len()-2);
    //     return  changed_text.chars().rev().collect::<String>()+"quay"
    // }
    for (i, v) in text.to_string().chars().enumerate() {
        if i == 0 {
            match v {
                'a' | 'e' | 'i' | 'o' | 'u' => return text.to_string() + "ay",
                _ => 
                {
                    iterate.next();
                    if iterate.next()==Some('q')&&iterate.next()==Some('u'){
                orig.truncate(3);

                let mut changed_text:String = text.to_string().chars().rev().collect();
                changed_text.truncate(text.len()-3);
                return changed_text.chars().rev().collect::<String>() +&orig+ "ay";

                }else{
                    continue;
                }},
            };
        }
        match v {
            'a' | 'e' | 'i' | 'o' | 'u' => {
                orig.truncate(i);
                let mut changed_text:String = text.to_string().chars().rev().collect();
                changed_text.truncate(text.len()-i);
                return changed_text.chars().rev().collect::<String>() +&orig + "ay";
            }
            _ => continue,
        }
    }
    "".to_string()
}
/*
If a word begins with a vowel, just add "ay" to the end.
If it begins with a consonant, then we take all consonants before the first vowel, move them to the end of the word, and then add "ay" at the end.
If a word starts with a consonant followed by "qu", move it to the end of the word, and then add an "ay" at the end.
Only the latin vowels will be considered as vowels (aeiou).
 */
