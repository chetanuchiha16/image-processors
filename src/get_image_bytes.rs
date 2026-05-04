use std::{fs, io::Error, path::PathBuf};

pub fn get_image_paths(image_path: &str) -> Result<Vec<PathBuf>, Error> {
    // let image_dir = fs::read_dir(image_path)?;
    // let image_paths =
    fs::read_dir(image_path)?
        // .into_iter()
        // .filter_map(|p| p.ok())
        .map(|res| {
            // let x = x?;
            res.map(|e: fs::DirEntry| e.path())
            // Ok(x.path())
        })
        .filter(|res| {
            match res {
                Ok(path) => {
                    let ext = path.extension().and_then(|x| x.to_str());
                    // ext == Some("jpg") || ext == Some("png")
                    matches!(ext, Some("jpg") | Some("png"))
                }
                Err(_) => true,
            }
            // let extension = x.extension().and_then(|x| x.to_str());
        })
        .collect()
    // Ok(image_paths)
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
