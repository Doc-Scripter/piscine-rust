pub use core::num;
use std::f64;

pub fn transform_and_save_on_heap(s: String) -> Box<Vec<u32>> {
    let nums_str=s.split_ascii_whitespace();
    let mut nums:Vec<u32>=vec![];
    for val in nums_str{
        if val.ends_with("k"){
            let new_str=val.trim_end_matches("k");
            let  num=new_str.parse::<f64>().unwrap_or_default();
            nums.push((num*1000.0) as u32);

        }else{
            nums.push(val.parse().unwrap_or_default());

        }
    }
    Box::new(nums)

}
pub fn take_value_ownership(a: Box<Vec<u32>>) -> Vec<u32> {
a.get(0..=a.len()-1).unwrap().to_vec()
}