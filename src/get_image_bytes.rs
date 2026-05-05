use std::{fs, io::Error, path::PathBuf};

pub fn get_image_paths(image_path: &str) -> Result<Vec<PathBuf>, Error> {
    let paths =  fs::read_dir(image_path)?
    .filter_map(|entry_res| {
        let entry = entry_res.ok()?;
        let path = entry.path();
        let ext = path.extension()?.to_str();
        if matches!(ext, Some("jpg") | Some("png") | Some("jpeg")) {
            Some(path)
        } else {
            None
        }
    }).collect();
    Ok(paths)
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
