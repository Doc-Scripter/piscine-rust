pub fn factorial(num: u64) -> u64 {
    let mut num1:u64=1;
    let mut result:u64=1;

    loop{
        if result<num{
            result +=1;
            num1*=result;
        }else{
            break
        }
    }
    num1
}