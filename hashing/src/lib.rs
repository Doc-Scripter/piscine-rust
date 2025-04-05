use std::collections::HashMap;

pub fn mean(list: &[i32]) -> f64 {
    let sum: f64 = list.iter().sum::<i32>() as f64;
    (sum / list.len() as f64) as f64
}

pub fn median(list: &[i32]) -> i32 {
    // let mid;
    let mut  new_list =list.to_vec();
    let len = list.len();
    new_list.sort();
    if len % 2 == 0 {
        // Even length: average of the two middle elements
        let mid_right = len / 2;
        let mid_left = mid_right - 1;
       return ((new_list[mid_left] as f64 + new_list[mid_right] as f64) / 2.0)as i32;
    } else {
        // Odd length: middle element
        let mid = len / 2;
        return new_list[mid]
    }
}

pub fn mode(list: &[i32]) -> i32 {
    let mut counts = HashMap::new();
    let mut max =0;
    for i in list.iter(){
        *counts.entry(i).or_insert(1)+=1;
    }
    for v in counts.values(){
        if v>&max{
            max = *v;
        }
    }
    for (k,v) in counts.iter(){
        if v==&max{
            return **k;
        }
    }
    -1
}