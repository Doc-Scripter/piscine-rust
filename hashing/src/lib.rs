use std::collections::HashMap;

pub fn mean(list: &[i32]) -> f64 {
    let sum: f64 = list.iter().sum::<i32>() as f64;
    (sum / list.len() as f64) as f64
}

pub fn median(list: &[i32]) -> i32 {
    let mid;
    // if list.len() %2==0{
    //     mid=(list.len()/2)+1;

    // }else{
        mid=list.len()/2;
        
    // }
    let mut  new_list =list.to_vec();
    new_list.sort();
    new_list[mid]
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