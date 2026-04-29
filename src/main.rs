use std::io::Cursor;

use crate::{
    get_image_bytes::{get_encoded_image_bytes, get_image_paths},
    image_processors::{
        parallel_process_images, process_multiple_images, process_single_image,
        process_single_image_raw,
    },
};
use clap::Parser;

#[derive(Parser)]
#[command(author, version, about = "A fast parallel image processor")]
struct Args {
    /// Path to the directory containing images
    #[arg(default_value = "./src/images")]
    path: String,
}

use tracing_subscriber::fmt::format::FmtSpan;
mod get_image_bytes;
mod image_processors;
fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt()
        .with_span_events(FmtSpan::CLOSE) // Logs when a function (span) finishes
        .init();

    let args = Args::parse();
    let image_paths = get_image_paths(&args.path)?;
    let encoded_image_bytes = get_encoded_image_bytes(&image_paths)?;
    let single_processed_image = process_single_image(&encoded_image_bytes[0])?;
    image::ImageReader::new(Cursor::new(single_processed_image))
        .with_guessed_format()?
        .decode()?
        .save("src/images/op/single_image.jpg")?;
    process_single_image_raw(&encoded_image_bytes[0])?;
    process_multiple_images(&encoded_image_bytes)?;
    parallel_process_images(&encoded_image_bytes)?;
    println!("processed {} images", image_paths.len());
    Ok(())
}
