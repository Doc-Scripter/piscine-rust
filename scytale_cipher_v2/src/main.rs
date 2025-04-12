use scytale_cipher_v2::*;
fn main() {
    println!("\"scytale Code\" size=6 -> {:?}", scytale_cipher(String::from("scytale Code"), 6));
    println!("\"scytale Code\" size=8 -> {:?}", scytale_cipher(String::from("scytale Code"), 8));
}
/*
"scytale Code" size=6 -> "sec yCtoadle"
"scytale Code" size=8 -> "sCcoydtea l e"
*/