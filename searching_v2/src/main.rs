use searching_v2::*;

fn main() {
    let ar = [1, 3, 4, 6, 8, 9, 11];
    let f = search(&ar, 6);
    println!(
        "the element 6 is in the position {:?} in the array {:?}",
        f, ar
    );
}
/*
the element 6 is in the position Some(3) in the array [1, 3, 4, 6, 8, 9, 11]
*/