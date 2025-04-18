use roman_numbers::RomanNumber;

fn main() {
	println!("{:?}", RomanNumber::from(32));
	println!("{:?}", RomanNumber::from(9));
	println!("{:?}", RomanNumber::from(45));
	println!("{:?}", RomanNumber::from(0));
}
/*

RomanNumber([X, X, X, I, I])
RomanNumber([I, X])
RomanNumber([X, L, V])
RomanNumber([Nulla])

*/