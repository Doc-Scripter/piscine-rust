#[derive(Debug, Clone, PartialEq)]
pub struct Store {
    pub products: Vec<(String, f32)>,
}

impl Store {
    pub fn new(products: Vec<(String, f32)>) -> Store {
        Store { products }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Cart {
    pub receipt: Vec<(String, f32)>,
}
impl Cart {
    pub fn new() -> Cart {
        Cart {
            receipt: Vec::<(String, f32)>::new(),
        }
    }
    pub fn insert_item(&mut self, s: &Store, ele: String) {
        let amount = s.products.iter().find(|x| *x.0 == ele).unwrap().1;
        self.receipt.push((ele, amount))
    }
    pub fn generate_receipt(&mut self) -> Vec<f32> {
        let mut res: Vec<(String, f32)> = self.receipt.clone();
        let amounts: Vec<f32> = res.iter().map(|x| x.1).collect();

        if amounts.len() >= 3 {
            let mut groups = vec![Vec::<f32>::new()];
            groups.remove(0);

            let sets = amounts.len() / 3;
            let mut count = 0;
            for _i in 0..sets {
                let next = count + 3;
                groups.push(amounts[count..next].to_vec());
                count += 3;
            }

            groups.iter_mut().for_each(|x| {
                x.sort_by(|a, b| a.total_cmp(b));
                let sum: f32 = x.iter().sum();
                let discount = x[0] / sum;
                x.iter_mut().for_each(|y| {
                    *y = ((*y - (*y * discount)) * 100.0).round() / 100.0; // Round to 2 decimal places
                });
            });

            // Flatten the groups back into a single vector
            let mut final_vec = Vec::<f32>::new();
            groups.iter().for_each(|x| {
                final_vec.append(&mut x.clone());
            });

            // Update the receipt with the discounted values
            for (i, amount) in final_vec.iter().enumerate() {
                res[i].1 = *amount;
            }
        }

        // Update the Cart's receipt
        self.receipt = res.clone();

        // Return only the discounted amounts
        res.iter().map(|x| x.1).collect()
    }
}
