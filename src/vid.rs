use std::str::FromStr;

use super::error::Error;

#[derive(Debug, PartialEq)]
pub struct VID {
    pub id: u32,
    pub checksum: u8,
}

impl FromStr for VID {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let id = s.parse::<u32>()?;

        match id {
            id if id > 1000 => Err(Error::Overflow),
            _ => Ok(VID::new(id))
        }
    }
}

impl VID {
    pub fn new(id: u32) -> VID {
        let checksum = (
            id / 1000
            + 10 * ((id / 100) % 10)
            + 100 * ((id / 10) % 10)
            + 1000 * (id % 10)
        ) % 97;

        VID { id, checksum: checksum as u8 }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse() {
        let expected = VID::new(400);
        assert_eq!("400".parse::<VID>().unwrap(), expected);
    }

    #[test]
    fn parse_not_a_number() {
        let expected = Err(Error::Parse);
        assert_eq!("foo".parse::<VID>(), expected);
    }

    #[test]
    fn parse_overflow() {
        let expected = Err(Error::Overflow);
        assert_eq!("1001".parse::<VID>(), expected);
    }

    #[test]
    fn checksum() {
        assert_eq!(VID::new(1337).checksum, 56);
    }
}
