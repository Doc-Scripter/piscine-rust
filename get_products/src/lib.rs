pub fn get_products(arr: Vec<usize>) -> Vec<usize> {
    arr.iter().map(| x| {
        let mut prod = 1 as usize;
        arr.iter().for_each(|y| {
            if y != x {
                prod *= y;
            };
        });
        prod
    }).collect()
}
