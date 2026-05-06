use std::{fs, io::Error, path::PathBuf};

/// Scans a directory for image files with supported extensions (.jpg, .png, .jpeg).
///
/// # Arguments
/// * `image_path` - A string slice that holds the path to the directory to scan.
///
/// # Returns
/// * `Result<Vec<PathBuf>, Error>` - A vector of paths to the found images, or an IO error.
pub fn get_image_paths(image_path: &str) -> Result<Vec<PathBuf>, Error> {
    let paths = fs::read_dir(image_path)?
        .filter_map(|entry_res| {
            let entry = entry_res.ok()?;
            let path = entry.path();
            let ext = path.extension()?.to_str();
            if matches!(ext, Some("jpg") | Some("png") | Some("jpeg")) {
                Some(path)
            } else {
                None
            }
        })
        .collect();
    Ok(paths)
}

/// Reads the contents of multiple image files into memory as raw bytes.
///
/// # Arguments
/// * `paths` - A slice of `PathBuf` containing the paths to the images to be read.
///
/// # Returns
/// * `Result<Vec<Vec<u8>>, Error>` - A vector where each element is a `Vec<u8>` containing
///   the raw encoded bytes of an image, or an IO error.
pub fn get_encoded_image_bytes(paths: &[PathBuf]) -> Result<Vec<Vec<u8>>, Error> {
    paths
        .iter()
        .map(|path| {
            let byte = fs::read(path)?;
            Ok(byte)
        })
        .collect()
}
