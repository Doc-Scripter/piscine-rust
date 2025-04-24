use lifetimes::*;

fn main() {
	let person = Person::new("Leo");

	println!("Person = {:?}", person);
}
/*

Person = Person { name: "Leo", age: 0 }

*/