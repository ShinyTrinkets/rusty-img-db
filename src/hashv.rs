use image::GrayImage;
use std::fmt;
use strum_macros::EnumString;

/// Visual/Perceptual hash
pub fn hash_v(h: &HashV, image: GrayImage) -> String {
    match h {
        HashV::Ahash => format!("{:x}", ahash(image)),
        HashV::Dhash => format!("{:x}", dhash(image)),
    }
}

/**
 * Calculate the ahash of the provided prepared image.
 *
 * Returns a u128 representing the value of the hash
 *
 * Ref: https://github.com/warricksothr/Perceptual-Image-Hashing/blob/master/src/hash/ahash.rs
 * Originally by Drew Short
 */
fn ahash(image: GrayImage) -> u128 {
    let (width, height) = image.dimensions();

    // calculate the average pixel value
    let mut total = 0u64;
    for pixel in image.pixels() {
        total += pixel.0[0] as u64;
    }
    let mean = total / (height * width) as u64;

    // calculate a hash based on the mean
    let mut hash = 0u128;
    for pixel in image.pixels() {
        if pixel.0[0] as u64 >= mean {
            hash |= 1;
        } else {
            hash |= 0;
        }
        hash <<= 1;
    }

    // println!("Total: {} Mean: {} AHash: {}", total, mean, hash);
    hash
}

/**
 * Calculate the dhash of the provided prepared image
 *
 * Returns a u128 representing the value of the hash
 *
 * Ref: https://github.com/warricksothr/Perceptual-Image-Hashing/blob/master/src/hash/dhash.rs
 * Originally by Drew Short
 */
fn dhash(image: GrayImage) -> u128 {
    let first_pixel = image.pixels().nth(0).unwrap();
    let last_pixel = image.pixels().last().unwrap();
    let first_pixel_value = first_pixel.0[0] as u64;
    let last_pixel_value = last_pixel.0[0] as u64;

    let mut previous_pixel_value = 0u64;
    let mut hash = 0u128;
    for (x, y, pixel) in image.enumerate_pixels() {
        if x == 0 && y == 0 {
            previous_pixel_value = pixel.0[0] as u64;
            continue;
        }
        let pixel_val = pixel.0[0] as u64;
        if pixel_val >= previous_pixel_value {
            hash |= 1;
        } else {
            hash |= 0;
        }
        hash <<= 1;
        previous_pixel_value = first_pixel_value;
    }

    if first_pixel_value >= last_pixel_value {
        hash |= 1;
    } else {
        hash |= 0;
    }

    // println!("DHash: {}", hash);
    hash
}

/// Allowed visual hashes
#[derive(Clone, PartialEq, Eq, Debug, EnumString)]
#[strum(ascii_case_insensitive)]
pub enum HashV {
    Ahash,
    Dhash,
}

impl fmt::Display for HashV {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Debug::fmt(self, f)
    }
}
