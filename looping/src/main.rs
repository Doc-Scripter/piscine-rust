use std::io::{self, stdin};

fn main() -> io::Result<()> {
    loop {
        println!(
            " I am the beginning of the end, and the end of time and space. I am essential to creation, and I surround every place. What am I?"
        );
        let mut input = String::new();
        stdin().read_line(&mut input)?;
        if input.trim() == "The letter e" {
            break;
        }
    }
    Ok(())
}
