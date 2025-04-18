pub fn num_to_ordinal(x: u32) -> String {
    let num=x.to_string();
    match num.chars().last(){
        Some('1')=>num+"st",
        Some('2')=>if x>20{num+"nd"}else{num+"th"},
        Some('3')=>num+"rd",
        Some('4'..'9'|'0')=>num+"th",
        _ =>num
    }

}