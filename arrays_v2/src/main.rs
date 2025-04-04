use arrays_v2::*;

fn main() {
    let a: [i32; 10] = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    let b = [5;10];

    println!("The sum of the elements in {:?} is {}", a, sum(&a));
    println!("The sum of the elements in {:?} is {}", b, sum(&b));
    println!(
        "Array of {} elements filled with 10 = {:?}",
        thirtytwo_tens().len(),
        thirtytwo_tens()
    );
}