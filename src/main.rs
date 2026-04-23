use crate::{
    get_image_bytes::{get_encoded_image_bytes, get_image_paths},
    image_processors::{parallel_process_images, process_multiple_images, process_single_image},
};

mod get_image_bytes;
mod image_processors;
fn main() -> Result<(), Box<dyn std::error::Error>>{
    let image_paths = get_image_paths("./images")?;
    let encoded_image_bytes = get_encoded_image_bytes(&image_paths)?;
    process_single_image(&encoded_image_bytes[0])?;
    process_multiple_images(&encoded_image_bytes)?;
    parallel_process_images(&encoded_image_bytes)?;
    println!("Hello, world!");
    Ok(())
}
