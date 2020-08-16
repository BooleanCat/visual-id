use std::str::FromStr;
use std::fmt;

use super::error::Error;

#[derive(Debug, PartialEq, Copy, Clone)]
pub struct VID {
    pub id: u32,
    pub checksum: u8,
}

impl FromStr for VID {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        VID::new(s.parse()?)
    }
}

impl fmt::Display for VID {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:04}", self.id)
    }
}

impl VID {
    pub fn new(id: u32) -> Result<VID, Error> {
        if id > 9999 {
            return Err(Error::Overflow);
        }

        let checksum = (
            id / 1000
            + 10 * ((id / 100) % 10)
            + 100 * ((id / 10) % 10)
            + 1000 * (id % 10)
        ) % 97;

        Ok(VID { id, checksum: checksum as u8 })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse() {
        let expected = VID::new(4000).unwrap();
        assert_eq!("4000".parse::<VID>().unwrap(), expected);
    }

    #[test]
    fn parse_not_a_number() {
        let expected = Err(Error::Parse);
        assert_eq!("foo".parse::<VID>(), expected);
    }

    #[test]
    fn parse_overflow() {
        let expected = Err(Error::Overflow);
        assert_eq!("10001".parse::<VID>(), expected);
    }

    #[test]
    fn checksum() {
        assert_eq!(VID::new(1337).unwrap().checksum, 56);
    }

    #[test]
    fn display_pads_zeros() {
        assert_eq!("0022", format!("{}", VID::new(0022).unwrap()))
    }
}
