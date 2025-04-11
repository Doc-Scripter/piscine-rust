pub fn rotate(input: &str, key: i8) -> String {
    let mut res = "".to_string();
    if key > 0 {
        for val in input.chars() {
            if val.is_alphabetic() {
                if val.is_ascii_uppercase() {
                    if key as u64 % 26 == 0 {
                        res.push(val);
                        continue;
                    }
                    if val as u64 + key as u64 > 'Z' as u64 {
                        let diff = key as u64 - ('Z' as u64 - val as u64);
                        res.push(('A' as u64 + diff) as u8 as char);
                    } else {
                        res.push((val as u64 + key as u64) as u8 as char);
                    }
                } else {
                    if key as u64 % 26 == 0 {
                        res.push(val);
                        continue;
                    }
                    if val as u64 + key as u64 > 'z' as u64 {
                        let diff = key as u64 - ('z' as u64 - val as u64);
                        res.push(('a' as u64 + diff - 1) as u8 as char);
                    } else {
                        res.push((val as u64 + key as u64) as u8 as char);
                    }
                }
            } else {
                res.push(val);
            }
        }
    } else {
        let new_key = key * -1;

        for val in input.chars() {
            if val.is_alphabetic() {
                if val.is_ascii_uppercase() {
                    if 'A' as u64 > val as u64 - new_key as u64 {
                        let diff = 'A' as u64 - (val as u64 - new_key as u64);
                        res.push(('Z' as u64 - diff + 1) as u8 as char);
                    } else {
                        res.push((val as u64 - new_key as u64) as u8 as char);
                    }
                } else {
                    if 'a' as u64 > val as u64 - new_key as u64 {
                        // println!("{'a'}");
                        let diff = 'a' as u64 - (val as u64 - new_key as u64);
                        res.push(('z' as u64 - diff + 1) as u8 as char);
                    } else {
                        res.push((val as u64 - new_key as u64) as u8 as char);
                    }
                }
            } else {
                res.push(val);
            }
        }
    }
    res
}
