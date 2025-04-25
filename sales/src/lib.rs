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
    pub items: Vec<(String, f32)>,
   pub  receipt: Vec<f32>,
}
impl Cart {
    pub fn new() -> Cart {
        Cart {
            items: Vec::<(String, f32)>::new(),
            receipt: Vec::<f32>::new(),
        }
    }
    pub fn insert_item(&mut self, s: &Store, ele: String) {
        let amount = s.products.iter().find(|x| *x.0 == ele).unwrap().1;
        self.items.push((ele, amount))
    }
    pub fn generate_receipt(&mut self) -> Vec<f32> {
        let  res: Vec<f32> = self.items.iter().map(|x| x.1).collect();

        if res.len() >= 3 {
            let mut groups = Vec::new();
            let sets = res.len() / 3;
            
            // Split into groups of 3
            for i in 0..sets {
                let start = i * 3;
                groups.push(res[start..start + 3].to_vec());
            }

            // Process each group
            let mut final_vec = Vec::new();
            for mut group in groups {
                group.sort_by(|a, b| a.total_cmp(b));
                let sum: f32 = group.iter().sum();
                let discount = group[0] / sum;
                
                for val in group.iter_mut() {
                    *val = ((*val - (*val * discount)) * 100.0).round() / 100.0;
                }
                final_vec.extend(group);
            }

            // Add remaining items
            let remaining = res.len() % 3;
            if remaining > 0 {
                final_vec.extend(&res[sets * 3..]);
            }

            // Sort and update receipt
            final_vec.sort_by(|a, b| a.total_cmp(b));
            self.receipt = final_vec.clone();
            final_vec
        } else {
            self.receipt = res.clone();
            res
        }
    }
}
