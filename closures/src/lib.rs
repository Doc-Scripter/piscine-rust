pub fn first_fifty_even_square() -> Vec<i32> {
    let mut res=vec![];
    (1..=10000).for_each(|x:i32|{
        if x%2==0&&res.len()!=50{
            res.push(x.pow(2));
        }
    });
    res
}