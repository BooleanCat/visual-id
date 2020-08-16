use super::vid::VID;

pub struct VIDDigitIter {
    value: u32,
    mask: u32,
}

impl VIDDigitIter {
    pub fn new(value: VID) -> VIDDigitIter {
        VIDDigitIter { value: value.checksum as u32 * 10000 + value.id as u32, mask: 100000 }
    }
}

impl Iterator for VIDDigitIter {
    type Item = u8;

    fn next(&mut self) -> Option<Self::Item> {
        if self.mask == 0 {
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
        let want = vec![5, 6, 1, 3, 3, 7];
        let got: Vec<_> = VIDDigitIter::new(VID::new(1337).unwrap()).collect();
        assert_eq!(want, got);
    }

    #[test]
    fn iter_0() {
        let want: Vec<u8> = vec![0, 0, 0, 0, 0, 0];
        let got: Vec<_> = VIDDigitIter::new(VID::new(0000).unwrap()).collect();
        assert_eq!(want, got);
    }
}