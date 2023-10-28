mod convert;
use clap::Parser;
use image::DynamicImage;
use std::{path::Path, process};

/// Img2Ascii
///
/// Converts images into ASCII text
#[derive(Parser)]
pub struct Args {
    /// Print using ANSI color codes, instead of greyscale ASCII characters
    #[arg(short, long)]
    pub color: bool,
    /// Filename of image to convert
    pub filename: String,
}

fn main() {
    let args = Args::parse();
    let image = open_image(&args.filename);
    let ascii = convert::image_to_ascii(image, args.color);
    println!("{}", ascii);
}

/// Open an image, or exit process with a message if anything fails
fn open_image(filename: &str) -> DynamicImage {
    if !Path::new(filename).exists() {
        eprintln!("Cannot open image - File does not exist");
        process::exit(1);
    }
    match image::open(filename) {
        Ok(image) => image,
        Err(error) => {
            eprintln!("Cannot open image - {}", error);
            process::exit(2);
        }
    }
}
