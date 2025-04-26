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
        let prices: Vec<f32> = self.items.iter().map(|x| x.1).collect();
        
        if prices.len() >= 3 {
            let mut final_prices = Vec::new();
            let sets = prices.len() / 3;
            
            for i in 0..sets {
                let mut group: Vec<f32> = prices[i*3..i*3+3].to_vec();
                
                // Sort the group to find the cheapest item
                group.sort_by(|a, b| a.total_cmp(b));
                
                let cheapest = group[0];
                let total = group.iter().sum::<f32>();
                
                // Calculate the discount percentage (cheapest item / total)
                let discount_percentage = cheapest / total;
                
                // Apply the discount to each item in the group
                for &price in &group {
                    let discounted = price * (1.0 - discount_percentage);
                    // Round to exactly 2 decimal places
                    let rounded = (discounted * 100.0).round() / 100.0;
                    final_prices.push(rounded);
                }
            }
            
            // Add any remaining items without discount
            for i in (sets * 3)..prices.len() {
                final_prices.push((prices[i] * 100.0).round() / 100.0);
            }
            
            // Sort the final prices
            final_prices.sort_by(|a, b| a.total_cmp(b));
            self.receipt = final_prices.clone();
            final_prices
        } else {
            // If less than 3 items, no discount
            let rounded_prices: Vec<f32> = prices.iter()
                .map(|&x| (x * 100.0).round() / 100.0)
                .collect();
            self.receipt = rounded_prices.clone();
            rounded_prices
        }
    }
}

