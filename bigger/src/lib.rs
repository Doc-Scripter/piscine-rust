use std::collections::HashMap;

pub fn bigger(h: HashMap<&str, i32>) -> i32 {
  let vals = h.values();
  let mut max = 0;
  for v in vals{
    if v>&max{
        max=*v
    }
  }
  max
}