
fn main() {
    ["hello there", "", "you are stupid", "stupid"]
        .into_iter()
        .for_each(|m| println!("{:?}", profanity_filter_another_v2::check_ms(m)));
}
/*
Ok("hello there")
Err("ERROR: illegal")
Err("ERROR: illegal")
Err("ERROR: illegal")
*/