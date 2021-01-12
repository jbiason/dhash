/*
DHash - a perceptual hashing program for images.
Copyright (C) 2020-2021  Julio Biason

This program is free software: you can redistribute it and/or modify
it under the terms of the GNU Affero General Public License as published by
the Free Software Foundation, either version 3 of the License, or
(at your option) any later version.

This program is distributed in the hope that it will be useful,
but WITHOUT ANY WARRANTY; without even the implied warranty of
MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
GNU Affero General Public License for more details.

You should have received a copy of the GNU Affero General Public License
along with this program.  If not, see <https://www.gnu.org/licenses/>.
*/

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
    let image_bytes = image.to_bytes();
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
