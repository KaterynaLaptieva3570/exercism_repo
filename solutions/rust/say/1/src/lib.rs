pub fn encode(n: u64) -> String {
    if n == 0 {
        return "zero".to_string();
    }

    const ONES: [&str; 20] = [
        "", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
        "ten", "eleven", "twelve", "thirteen", "fourteen", "fifteen",
        "sixteen", "seventeen", "eighteen", "nineteen",
    ];
    const TENS: [&str; 10] = [
        "", "", "twenty", "thirty", "forty", "fifty",
        "sixty", "seventy", "eighty", "ninety",
    ];
    const SCALES: [&str; 7] = [
        "", "thousand", "million", "billion",
        "trillion", "quadrillion", "quintillion",
    ];

    fn below_thousand(n: u64) -> String {
        match n {
            0 => String::new(),
            1..=19 => ONES[n as usize].to_string(),
            20..=99 => {
                let t = TENS[(n / 10) as usize];
                let o = n % 10;
                if o == 0 {
                    t.to_string()
                } else {
                    format!("{}-{}", t, ONES[o as usize])
                }
            }
            100..=999 => {
                let h = ONES[(n / 100) as usize];
                let rest = n % 100;
                if rest == 0 {
                    format!("{} hundred", h)
                } else {
                    format!("{} hundred {}", h, below_thousand(rest))
                }
            }
            _ => unreachable!(),
        }
    }

    let mut parts = Vec::new();
    let mut value = n;
    let mut scale = 0;

    while value > 0 {
        let chunk = (value % 1_000) as u64;
        if chunk != 0 {
            let mut s = below_thousand(chunk);
            if scale > 0 {
                s.push(' ');
                s.push_str(SCALES[scale]);
            }
            parts.push(s);
        }
        value /= 1_000;
        scale += 1;
    }

    parts.reverse();
    parts.join(" ")
}