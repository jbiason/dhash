use image;

pub enum DhashError {
    FileDoesNotExist,
}

impl std::convert::From<image::error::ImageError> for DhashError {
    fn from(_: image::error::ImageError) -> DhashError {
        DhashError::FileDoesNotExist
    }
}

pub fn dhash(filename: &str) -> Result<String, DhashError> {
    let image = image::open(filename)?
        .resize_exact(9, 8, image::imageops::FilterType::Nearest)
        .grayscale();
    let image_bytes = image
        .to_bytes();
    let mut all_hash: u64 = 0;
    for byte in 0..8 {
        let line = byte * 9;
        for bits in 0..8 {
            all_hash <<= 1;
            if image_bytes[line + bits] > image_bytes[line + bits + 1] {
                all_hash = all_hash | 0b00000001;
            }
        }
    }

    Ok(format!("{:#018x}", all_hash).into())
}
