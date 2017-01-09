use std::fmt;

static ROMAN_MAP: [(usize, &'static str); 13] = [(1000, "M"),
                                                (900, "CM"),
                                                (500, "D"),
                                                (400, "CD"),
                                                (100, "C"),
                                                (90, "XC"),
                                                (50, "L"),
                                                (40, "XL"),
                                                (10, "X"),
                                                (9, "IX"),
                                                (5, "V"),
                                                (4, "IV"),
                                                (1, "I")];

pub struct Roman(usize);

impl fmt::Display for Roman {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut n = self.0;
        let mut roman = String::new();

        for &(key, val) in ROMAN_MAP.into_iter() {
            while n >= key {
                roman.push_str(val);
                n = n - key;
            }
        }
        write!(f, "{}", roman)
    }
}

impl Roman {
    pub fn from(num: usize) -> Roman {
        Roman(num)
    }
}
