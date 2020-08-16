use std::iter::repeat;
use std::io::Write;

use super::vid::VID;
use super::digit::Digits;
use super::error::Error;

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

impl From<VID> for Image {
    fn from(vid: VID) -> Image {

        // Reserved bits
        let data = repeat(0).take(1)

            // Visual ID
            .chain(Digits::new(vid.checksum).map(|d| Segments::from(d).0))
            .chain(Digits::new(vid.id).map(|d| Segments::from(d).0))

            // Reserved bits
            .chain(repeat(0).take(25))
            .collect();

        Image { data }
    }
}

impl Image {
    pub fn encode<W: Write> (&self, writer: W) -> Result<(), Error> {

        let mut encoder = png::Encoder::new(writer, 256, 1);
        encoder.set_color(png::ColorType::Grayscale);
        encoder.set_depth(png::BitDepth::One);

        let mut writer = encoder.write_header()?;

        writer.write_image_data(&self.data)?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::File;
    use std::io::prelude::*;

    #[test]
    fn image() {
        let mut got = std::io::Cursor::new(Vec::new());

        Image::from(VID::new(1337)).encode(&mut got).unwrap();
        got.seek(std::io::SeekFrom::Start(0)).unwrap();

        let mut fixture = File::open("fixtures/1337.png").unwrap();
        let mut want = Vec::new();
        fixture.read_to_end(&mut want).unwrap();

        assert_eq!(want, got.into_inner());
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
