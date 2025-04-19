use lalgebra_vector::*;

fn main() {
	let vector_1: Vector<i64> = Vector(vec![1, 3, -5]);
	let vector_2: Vector<i64> = Vector(vec![4, -2, -1]);
	println!("{:?}", vector_1.dot(&vector_2));
	println!("{:?}", vector_1 + vector_2);
}
/*
Some(3)
Some(Vector([5, 1, -6]))

*/

/*
  --> src/main.rs:60:13
   |
57 |         test_meta.assert_with_message(
   |                   ------------------- arguments to this method are incorrect
...
60 |             Some(Vector(vec![5i64, 1, -6])),
   |             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected `Vector<i64>`, found `Option<Vector<i64>>`
   |
   = note: expected struct `lalgebra_vector::Vector<_>`
                found enum `Option<lalgebra_vector::Vector<_>>`
note: method defined here
  --> /jail/tests/lib/src/lib.rs:33:12
   |
33 |     pub fn assert_with_message<U: std::fmt::Debug + std::cmp::PartialEq>(
   |            ^^^^^^^^^^^^^^^^^^^

error[E0308]: mismatched types
  --> src/main.rs:68:13
   |
65 |         test_meta.assert_with_message(
   |                   ------------------- arguments to this method are incorrect
...
68 |             None,
   |             ^^^^ expected `Vector<i64>`, found `Option<_>`
   |
   = note: expected struct `lalgebra_vector::Vector<i64>`
                found enum `Option<_>`
note: method defined here
  --> /jail/tests/lib/src/lib.rs:33:12
   |
33 |     pub fn assert_with_message<U: std::fmt::Debug + std::cmp::PartialEq>(
   |            ^^^^^^^^^^^^^^^^^^^


*/