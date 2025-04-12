use pig_latin_v2::*;

fn main() {
    println!("{}", pig_latin(&String::from("igloo")));
    println!("{}", pig_latin(&String::from("apple")));
    println!("{}", pig_latin(&String::from("hello")));
    println!("{}", pig_latin(&String::from("square")));
    println!("{}", pig_latin(&String::from("xenon")));
    println!("{}", pig_latin(&String::from("chair")));
    println!("want= ueenqay \ngot= {} ", pig_latin(&String::from("queen")));
}      
/*
iglooay
appleay
ellohay
aresquay
enonxay
airchay
ueenqay
*/