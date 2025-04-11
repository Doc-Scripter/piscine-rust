use scores_v2::*;

fn main() {
    println!("{}", score("a"));
    println!("{}", score("ã ê Á?"));
    println!("{}", score("ThiS is A Test"));
}
/*
1
0
14
*/