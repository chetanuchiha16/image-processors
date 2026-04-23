use std::{fs, io::Error, path::PathBuf};

pub fn get_image_paths(image_path: &str) -> Result<Vec<PathBuf>, Error> {
    let image_dir = fs::read_dir(image_path)?;
    let image_paths = image_dir
        .into_iter()
        .filter_map(|p| p.ok())
        .map(|x| x.path())
        .filter(|x| {
            x.extension().and_then(|x| x.to_str()) == Some("jpg")
                || x.extension().and_then(|x| x.to_str()) == Some("png")
        })
        .collect();
    Ok(image_paths)
}

pub fn get_encoded_image_bytes(paths: &[PathBuf]) -> Result<Vec<Vec<u8>>, Error> {
    paths
        .iter()
        .map(|path| {
            let byte = fs::read(path)?;
            Ok(byte)
        })
        .collect()
}
