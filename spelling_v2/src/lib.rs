pub fn spell(n: u64) -> String {
    let units = [
        "", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine", "ten",
        "eleven", "twelve", "thirteen", "fourteen", "fifteen", "sixteen", "seventeen",
        "eighteen", "nineteen",
    ];
    let tens = [
        "", "", "twenty", "thirty", "forty", "fifty", "sixty", "seventy", "eighty", "ninety",
    ];

    if n == 0 {
        return "zero".to_string();
    }

    if n >= 1_000_000 {
        return "one million".to_string();
    }

    if n >= 1000 {
        let thousands = n / 1000;
        let remainder = n % 1000;
        if remainder == 0 {
            return format!("{} thousand", spell(thousands));
        }
        return format!("{} thousand {}", spell(thousands), spell(remainder));
    }

    if n >= 100 {
        let hundreds = n / 100;
        let remainder = n % 100;
        if remainder == 0 {
            return format!("{} hundred", units[hundreds as usize]);
        }
        return format!("{} hundred {}", units[hundreds as usize], spell(remainder));
    }

    if n >= 20 {
        let ten = n / 10;
        let unit = n % 10;
        if unit == 0 {
            return tens[ten as usize].to_string();
        }
        return format!("{}-{}", tens[ten as usize], units[unit as usize]);
    }

    units[n as usize].to_string()
}