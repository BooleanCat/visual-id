mod vid;
mod error;
mod image;
mod digit;

use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

use structopt::{StructOpt, clap};

use vid::VID;
use image::Image;

#[derive(StructOpt)]
#[structopt(name = "visual-id")]
struct Opt {
    #[structopt(name = "IDS-FILE")]
    ids_file: String,
}

fn main() {
    let opts = Opt::from_args();

    let file = File::open(opts.ids_file).unwrap_or_else(|err| {
        clap::Error::with_description(&format!(r#"open file: {}"#, err), clap::ErrorKind::Io).exit();
    });

    let ids = BufReader::new(file).lines().collect::<Result<Vec<_>, _>>().unwrap_or_else(|err| {
        clap::Error::with_description(&format!(r#"read file: {}"#, err), clap::ErrorKind::Io).exit();
    });

    let vids = ids.iter().map(|id| id.parse::<VID>()).collect::<Result<Vec<_>, _>>().unwrap_or_else(|err| {
        clap::Error::with_description(&format!(r#"parse id: {:?}"#, err), clap::ErrorKind::Io).exit();
    });

    let _: Vec<_> = vids.into_iter().map(|vid| (vid, Image::from(vid))).map(|(vid, image)| {
        let image_file = File::create(format!("{}/{}.png", "output", vid)).unwrap_or_else(|err| {
            clap::Error::with_description(&format!(r#"create file: {:?}"#, err), clap::ErrorKind::Io).exit();
        });
        image.encode(image_file).unwrap_or_else(|err| {
            clap::Error::with_description(&format!(r#"encode image: {:?}"#, err), clap::ErrorKind::Io).exit();
        });
    }).collect();
}
