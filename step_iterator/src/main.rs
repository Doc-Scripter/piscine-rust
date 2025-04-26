use step_iterator::*;

fn main() {
	for v in StepIterator::new(0, 100, 10) {
		print!("{},", v);
	}
	println!();

	for v in StepIterator::new(0, 100, 12) {
		print!("{},", v)
	}
	println!();
}
/*

0,10,20,30,40,50,60,70,80,90,100,
0,12,24,36,48,60,72,84,96,

*/