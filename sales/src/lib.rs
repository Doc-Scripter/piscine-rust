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
    pub receipt: Vec<f32>,
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
        let res: Vec<f32> = self.items.iter().map(|x| x.1).collect();
        
        if res.len() >= 3 {
            let mut final_vec = Vec::new();
            let sets = res.len() / 3;
            
            for i in 0..sets {
                let mut group: Vec<f32> = res[i*3..i*3+3].to_vec();
                group.sort_by(|a, b| a.total_cmp(b));
                
                // The cheapest item is free, but we need to distribute its value
                // across all three items proportionally
                let cheapest = group[0];
                let discount_ratio = cheapest / group.iter().sum::<f32>();
                
                for price in group {
                    let discounted = price * (1.0 - discount_ratio);
                    final_vec.push(round_to_two_decimals(discounted));
                }
            }
            
            // Add any remaining items without discount
            for i in (sets * 3)..res.len() {
                final_vec.push(round_to_two_decimals(res[i]));
            }
            
            final_vec.sort_by(|a, b| a.total_cmp(b));
            self.receipt = final_vec.clone();
            final_vec
        } else {
            // If less than 3 items, no discount
            let rounded_res: Vec<f32> = res.iter().map(|&x| round_to_two_decimals(x)).collect();
            self.receipt = rounded_res.clone();
            rounded_res
        }
    }
}

fn round_to_two_decimals(value: f32) -> f32 {
    (value * 100.0).trunc() / 100.0
}