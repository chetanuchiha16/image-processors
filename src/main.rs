use crate::{
    get_image_bytes::get_encoded_image_bytes,
    image_processors::{parallel_process_images, process_multiple_images, process_single_image},
};

mod get_image_bytes;
mod image_processors;
fn main() {
    get_encoded_image_bytes();
    process_single_image();
    process_multiple_images();
    parallel_process_images();
    println!("Hello, world!");
}
