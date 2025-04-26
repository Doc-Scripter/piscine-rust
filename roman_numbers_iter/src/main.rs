use roman_numbers_iter::RomanNumber;

fn main() {
	let mut number = RomanNumber::from(15);

	println!("{:?}", number);
	println!("{:?}", number.next());
}
/*
RomanNumber([X, V])
Some(RomanNumber([X, V, I]))
*/