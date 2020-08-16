use std::iter::repeat;

use super::vid::VID;
use super::digit::Digits;

#[derive(Debug, PartialEq)]
pub struct Segments(u8);

impl From<u8> for Segments {
    fn from(value: u8) -> Self {
        Segments(match value {
            0 => 0b01110111,
            1 => 0b01000010,
            2 => 0b10110110,
            3 => 0b11010110,
            4 => 0b11000011,
            5 => 0b11010101,
            6 => 0b11110101,
            7 => 0b01000110,
            8 => 0b11110111,
            9 => 0b11010111,
            _ => unreachable!(),
        })
    }
}

pub struct Image {
    data: Vec<u8>,
}

impl From<&[VID]> for Image {
    fn from(vids: &[VID]) -> Image {

        // Reserved bits
        let data = repeat(0).take(1)

            // Visual ID
            .chain(
                vids.iter()
                .flat_map(|v| Digits::new(v.checksum).chain(Digits::new(v.id)))
                .map(|d| Segments::from(d).0)

            // Reserved bits
            ).chain(repeat(0).take(25))
            .collect();

        Image { data }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn image() {
        let got = Image::from(vec![VID::new(1337), VID::new(1337)].as_slice()).data;
        let want = vec![
            0b00000000, // 0
            0b11010101, // 5
            0b11110101, // 6
            0b01000010, // 1
            0b11010110, // 3
            0b11010110, // 3
            0b01000110, // 7
            0b11010101, // 5
            0b11110101, // 6
            0b01000010, // 1
            0b11010110, // 3
            0b11010110, // 3
            0b01000110, // 7
            0b00000000, // 0
            0b00000000, // 0
            0b00000000, // 0
            0b00000000, // 0
            0b00000000, // 0
            0b00000000, // 0
            0b00000000, // 0
            0b00000000, // 0
            0b00000000, // 0
            0b00000000, // 0
            0b00000000, // 0
            0b00000000, // 0
            0b00000000, // 0
            0b00000000, // 0
            0b00000000, // 0
            0b00000000, // 0
            0b00000000, // 0
            0b00000000, // 0
            0b00000000, // 0
            0b00000000, // 0
            0b00000000, // 0
            0b00000000, // 0
            0b00000000, // 0
            0b00000000, // 0
            0b00000000, // 0
        ];

        assert_eq!(want, got);
    }

    #[test]
    fn segments() {
        let want = vec![
            Segments(0b11010101), // 5
            Segments(0b11110101), // 6
            Segments(0b01000010), // 1
            Segments(0b11010110), // 3
            Segments(0b11010110), // 3
            Segments(0b01000110), // 7
        ];

        let got: Vec<_> = vec![5, 6, 1, 3, 3, 7].into_iter().map(Segments::from).collect();

        assert_eq!(want, got);
    }
}
