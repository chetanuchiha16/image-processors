use std::io::Cursor;

use image::{ImageError, ImageReader, error::ImageFormatHint, imageops};

pub fn process_single_image(encoded_image_bytes: &[u8]) -> Result<Vec<u8>, ImageError> {
    let reader = ImageReader::new(Cursor::new(encoded_image_bytes)).with_guessed_format()?;

    if let Some(image_format) = reader.format() {
        let image = reader.decode()?;
        let resised_image = image.resize(224, 224, imageops::Lanczos3);
        let mut buffer = Cursor::new(Vec::new());
        resised_image.write_to(&mut buffer, image_format)?;
        Ok(buffer.into_inner())
    } else {
        Err(ImageError::Unsupported(ImageFormatHint::Unknown.into()))
    }
}

pub fn process_multiple_images<T>(encoded_image_arrays: &[T]) -> Result<Vec<Vec<u8>>, ImageError>
where
    T: AsRef<[u8]> + Sync,
{
    let processed_images = encoded_image_arrays
        .iter()
        .map(|encoded_image_bytes| {
            let processed_image = process_single_image(encoded_image_bytes.as_ref())?;
            Ok(processed_image)
        })
        .collect();
    processed_images
}

pub fn parallel_process_images() {}
