#[derive(Debug)]
pub struct Numbers<'a> {
    numbers: &'a [u32],
}

impl <'a>Numbers <'a>{
    pub fn new(numbers: &'a [u32]) -> Self {
        Numbers{numbers}
    }

    pub fn list(&self) -> &[u32] {
        self.numbers
    }

    pub fn latest(&self) -> Option<u32> {
    match self.numbers.last()  {
        None => None,
        Some(num) => Some(*num)
    }
    }

    pub fn highest(&self) -> Option<u32> {
        let mut  max :u32=0;
        self.numbers.iter().for_each(|&x| if x>max {max=x} else {} );
        Some(max)
    }

    pub fn highest_three(&self) -> Vec<u32> {
        if self.numbers.len()<=3{
            let mut  max :Vec<u32>=vec![];
            let mut nums=self.numbers.to_vec();
            nums.sort();
            for _i in 0..self.numbers.len(){
                max.push(nums[nums.len()-1]);
                nums=nums[..nums.len()-1].to_vec();
            }
            max
        }else{
            let mut  max :Vec<u32>=vec![];
            let mut nums=self.numbers.to_vec();
            nums.sort();
            for _i in 0..3{
                max.push(nums[nums.len()-1]);
                nums=nums[..nums.len()-1].to_vec();
            }
            max
        }
    }
}