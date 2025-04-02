use to_url_v2::*;

fn main() {
    let s = "Hello, world!";
    println!("'{}' parsed as an URL becomes '{}'", s, to_url(s));
}
