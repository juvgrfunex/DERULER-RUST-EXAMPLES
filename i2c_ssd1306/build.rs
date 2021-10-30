use image::{imageops::FilterType, io::Reader as ImageReader};
use itertools::Itertools;
use std::{fs::File, io::Write, path::Path};

fn main() {
    convert_image("elmorlabs_logo.png", "primary_image.raw");
    convert_image("der8auer_logo.png", "secondary_image.raw");
}

fn convert_image<P>(input: P, output: P)
where
    P: AsRef<Path>,
{
    let img: Vec<u8> = ImageReader::open(input)
        .unwrap()
        .decode()
        .unwrap()
        .resize(64, 64, FilterType::Lanczos3)
        .to_luma8()
        .iter()
        .map(|grayscale| if *grayscale > 127 { 1u8 } else { 0u8 })
        .chunks(8)
        .into_iter()
        .map(|bits| {
            let mut byte: u8 = 0;
            for (i, bit) in bits.enumerate() {
                byte += bit << (7 - i);
            }
            byte
        })
        .collect();

    File::create(output).unwrap().write_all(&img).unwrap();
}
