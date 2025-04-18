use rot_v2::*;

fn main() {

    println!("The letter \"a\" becomes: {}", rotate("a", 26));
    println!("The letter \"m\" becomes: {}", rotate("m", 0));
    println!("The letter \"m\" becomes: {}", rotate("m", 13));
    println!("The letter \"a\" becomes: {}", rotate("a", 15));
    println!("The word \"MISS\" becomes: {}", rotate("MISS", 5));
    println!(
        "The decoded message is: {}",
        rotate("Gur svir obkvat jvmneqf whzc dhvpxyl.", 13)
    );
    println!(
        "The decoded message is: {}",
        rotate("Mtb vznhpqd ifky ozrunsl ejgwfx ajc", 5)
    );
    println!(
        "Your cypher wil be: {}",
        rotate("Testing with numbers 1 2 3", 4)
    );
    println!("Your cypher wil be: {}", rotate("Testing", -14));
    println!("The letter \"a\" becomes: {}", rotate("a", -1));
}
/*
The letter "a" becomes: a
The letter "m" becomes: m
The letter "m" becomes: z
The letter "a" becomes: p
The word "MISS" becomes: RNXX
The decoded message is: The five boxing wizards jump quickly.
The decoded message is: Ryg aesmuvi nkpd tewzsxq jolbkc foh
Your cypher wil be: Xiwxmrk amxl ryqfivw 1 2 3
Your cypher wil be: Fqefuzs
The letter "a" becomes: z
*/