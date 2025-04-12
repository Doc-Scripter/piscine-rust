use diamond_creation_v2::*;

fn main() {
    println!("{:?}", get_diamond('A'));
    println!("{:?}", get_diamond('C'));
    for line in get_diamond('C') {
        println!("{}", line);
    }
}
/*
["A"]
["  A  ", " B B ", "C   C", " B B ", "  A  "]
  A  
 B B 
C   C
 B B 
  A
*/