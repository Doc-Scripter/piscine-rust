use logic_number_v2::*;

fn main() {
    let array = [9, 10, 153, 154];
    for pat in &array {
        if number_logic(*pat) == true {
            println!(
                "this number returns {} because the number {} obey the rules of the sequence",
                number_logic(*pat),
                pat
            )
        }
        if number_logic(*pat) == false {
            println!("this number returns {} because the number {} does not obey the rules of the sequence", number_logic(*pat),pat )
        }
    }
}
/*
this number returns true because the number 9 obey the rules of the sequence
this number returns false because the number 10 does not obey the rules of the sequence
this number returns true because the number 153 obey the rules of the sequence
this number returns false because the number 154 does not obey the rules of the sequence

*/