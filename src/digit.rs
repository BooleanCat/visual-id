pub struct Digits {
    value: u64,
    mask: u64,
}

impl Digits {
    pub fn new<T: Into<u64>>(value: T) -> Digits {
        let value = value.into();
        let num_digits = (value as f64).log10() as usize;
        Digits { value, mask: 10_f64.powf(num_digits as f64) as u64 }
    }
}

impl Iterator for Digits {
    type Item = u8;

    fn next(&mut self) -> Option<Self::Item> {
        if self.mask == 0 || self.value == 0 {
            return None;
        }

        let digit = self.value / self.mask % 10;
        self.mask /= 10;

        Some(digit as u8)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn iter_1337() {
        let want = vec![1, 3, 3, 7];
        let got: Vec<_> = Digits::new(1337_u16).collect();
        assert_eq!(want, got);
    }

    #[test]
    fn iter_0() {
        let want: Vec<u8> = vec![];
        let got: Vec<_> = Digits::new(0_u16).collect();
        assert_eq!(want, got);
    }

    #[test]
    fn iter_5() {
        let want = vec![5];
        let got: Vec<_> = Digits::new(5_u16).collect();
        assert_eq!(want, got);
    }

    #[test]
    fn iter_560496() {
        let want = vec![5, 6, 0, 4, 9, 6];
        let got: Vec<_> = Digits::new(560496_u32).collect();
        assert_eq!(want, got);
    }
}