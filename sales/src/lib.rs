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
        let mut prices: Vec<f32> = self.items.iter().map(|x| x.1).collect();
        
        // Sort all prices ascendingly
        prices.sort_by(|a, b| a.total_cmp(b));
        
        let mut final_prices = Vec::new();
        let mut idx = 0;
        
        while idx + 3 <= prices.len() {
            let group = &prices[idx..idx + 3];
            let cheapest = group[0];
            let total: f32 = group.iter().sum();
            
            let discount_percentage = cheapest / total;
            
            for &price in group {
                let discounted = price * (1.0 - discount_percentage);
                let rounded = (discounted * 100.0).round() / 100.0;
                final_prices.push(rounded);
            }
            
            idx += 3;
        }
        
        // Handle any remaining items (less than 3)
        while idx < prices.len() {
            let rounded = (prices[idx] * 100.0).round() / 100.0;
            final_prices.push(rounded);
            idx += 1;
        }
        
        // Sort final prices before saving
        final_prices.sort_by(|a, b| a.total_cmp(b));
        
        self.receipt = final_prices.clone();
        final_prices
    }
    
}

