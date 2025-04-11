
pub fn number_logic(num: u32) -> bool {
    let  num_str = num.to_string();
    let nums = num_str.split("");
    let mut sum=0;
    for v in nums{
        let each_num=(v).parse::<i32>().unwrap_or(0) ;
        sum+=each_num.pow(num_str.len() as u32);
        // each_num as i32;
    }
    if num == sum as u32{

        true
    }else{
        false
    }

}