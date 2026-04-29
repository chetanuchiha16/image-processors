use image::{ImageError, ImageReader, error::ImageFormatHint, imageops};
use ndarray::Array3;
use rayon::prelude::*;
use std::{error::Error, io::Cursor};
use tracing::instrument;
/// takes encoded bytes of a single image and processes it and returns encoded bytes of the processed image
pub fn process_single_image(encoded_image_bytes: &[u8]) -> Result<Vec<u8>, ImageError> {
    let reader = ImageReader::new(Cursor::new(encoded_image_bytes)).with_guessed_format()?;

    if let Some(image_format) = reader.format() {
        let image = reader.decode()?;
        let resized_image = image.resize(224, 224, imageops::Lanczos3);
        let mut buffer = Cursor::new(Vec::new());
        resized_image.write_to(&mut buffer, image_format)?;
        Ok(buffer.into_inner())
    } else {
        Err(ImageError::Unsupported(ImageFormatHint::Unknown.into()))
    }
}

#[instrument(level = "info", skip_all)]
pub fn process_single_image_nd_array(encoded_image_bytes: &[u8]) -> Result<Array3<f32>, Box<dyn Error>> {
    let reader = ImageReader::new(Cursor::new(encoded_image_bytes)).with_guessed_format()?;
    let image = reader.decode()?;

    let resized_image = image.resize_exact(224, 224, imageops::Lanczos3);
    let rgb_image = resized_image.into_rgb8();
    let (height, width) = rgb_image.dimensions();
    let raw_image_byte = rgb_image.into_raw();
    let nd_array = Array3::from_shape_vec((height as usize, width as usize, 3), raw_image_byte)?
        .mapv(|x| x as f32 / 255.0);
    Ok(nd_array)
}

#[instrument(level = "info", skip_all)]
pub fn process_multiple_images<T>(encoded_image_bytes: &[T]) -> Result<Vec<Vec<u8>>, ImageError>
where
    T: AsRef<[u8]>,
{
    encoded_image_bytes
        .iter()
        .map(|single_encoded_image_bytes| {
            let processed_image = process_single_image(single_encoded_image_bytes.as_ref())?;
            Ok(processed_image)
        })
        .collect()
}

#[instrument(level = "info", skip_all)]
pub fn parallel_process_images<T>(encoded_image_bytes: &[T]) -> Result<Vec<Vec<u8>>, ImageError>
where
    T: AsRef<[u8]> + Sync,
{
    encoded_image_bytes
        .par_iter()
        .map(|single_encoded_image_bytes: &T| {
            let processed_image: Vec<u8> =
                process_single_image(single_encoded_image_bytes.as_ref())?;
            Ok(processed_image)
        })
        .collect()
}
