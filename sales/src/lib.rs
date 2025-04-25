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
            let mut final_vec = Vec::new();
            let sets = res.len() / 3;
            
            // Process complete sets of 3
            for i in 0..sets {
                let mut group: Vec<f32> = res[i*3..i*3+3].to_vec();
                group.sort_by(|a, b| a.total_cmp(b));
                
                let total: f32 = group.iter().sum();
                let discount_ratio = group[0] / total;
                
                for price in group {
                    let discounted = price * (1.0 - discount_ratio);
                    final_vec.push((discounted * 100.0).round() / 100.0);
                }
            }
            
            // Add remaining items
            final_vec.extend(&res[sets * 3..]);
            
            // Sort final result
            final_vec.sort_by(|a, b| a.total_cmp(b));
            self.receipt = final_vec.clone();
            final_vec
        } else {
            self.receipt = res.clone();
            res
        }
    }
}
