pub fn delete_and_backspace(s: &mut String) {
    let mut chars: Vec<char> = s.chars().collect();
    let mut i = 0;
    let mut j = chars.len()-1;
    // *s=chars.into_iter().rev().collect();
    while j >0{
        if chars[j] == '+'{
            chars.remove(j);
            j-=1
        }else{
            break
        }
    }
    loop{
    while i < chars.len() {
        if chars[i] == '-' {
           
            if i > 0 {
                chars.remove(i - 1);
                chars.remove(i - 1);
                i -= 1;
            } else {
               
                chars.remove(i);
            }
        } else if i+1<chars.len()&&chars[i] == '+'&&chars[i+1]!='+' {
           
            chars.remove(i);
            if i <= chars.len() {
                chars.remove(i);
            }
        } else {
           
            i += 1;
        }
    }
    if i>=chars.len()-1&&(chars.contains(&'-')||chars.contains(&'+')){
        i=0
    }else{
        break
    }
}
   
    *s = chars.into_iter().collect();
    
}

pub fn do_operations(v: &mut [String]) {
    for equation in v.iter_mut() {
        if equation.contains('+') {
            // Handle addition
            let parts: Vec<&str> = equation.split('+').collect();
            if parts.len() == 2 {
                let left = parts[0].trim().parse::<i32>().unwrap_or(0);
                let right = parts[1].trim().parse::<i32>().unwrap_or(0);
                *equation = (left + right).to_string();
            }
        } else if equation.contains('-') {
            // Handle subtraction
            let parts: Vec<&str> = equation.split('-').collect();
            if parts.len() == 2 {
                let left = parts[0].trim().parse::<i32>().unwrap_or(0);
                let right = parts[1].trim().parse::<i32>().unwrap_or(0);
                *equation = (left - right).to_string();
            }
        }
    }
}
