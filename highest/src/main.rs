use highest::*;

fn main() {
    let expected = [30, 500, 20, 70];
    let n = Numbers::new(&expected);
    println!("{:?}", n.list());
    println!("{:?}", n.highest());
    println!("{:?}", n.latest());
    println!("{:?}", n.highest_three());
}

/*

[30, 500, 20, 70]
Some(500)
Some(70)
[500, 70, 30]



*/