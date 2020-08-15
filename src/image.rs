use super::vid::VID;

pub struct Image {
    digits: Vec<u8>,
}

impl From<&[VID]> for Image {
    fn from(vids: &[VID]) -> Image {
        let mut digits = Vec::new();

        for vid in vids {
            let d = (vid.checksum / 10) as u8;

            digits.push(segments(d));

            let d = (vid.checksum % 10) as u8;

            digits.push(segments(d));

            let d = ((vid.id / 1_000) % 10) as u8;

            digits.push(segments(d));

            let d = ((vid.id / 100) % 10) as u8;

            digits.push(segments(d));

            let d = ((vid.id / 10) % 10) as u8;

            digits.push(segments(d));

            let d = (vid.id % 10) as u8;

            digits.push(segments(d));
        }

        Image { digits }
    }
}

fn segments(value: u8) -> u8 {
    match value {
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
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn image() {
        let expected = vec![
            0b11010101, // 5
            0b11110101, // 6
            0b01000010, // 1
            0b11010110, // 3
            0b11010110, // 3
            0b01000110, // 7
        ];

        assert_eq!(Image::from(vec![VID::new(1337)].as_slice()).digits, expected);
    }
}
