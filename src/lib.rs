use pyo3::prelude::*;

use crate::image_processors::ProcessorError;
pub mod get_image_bytes;
pub mod image_processors;

impl From<ProcessorError> for PyErr {
    fn from(err: ProcessorError) -> PyErr {
        pyo3::exceptions::PyValueError::new_err(err.to_string())
    }
}
/// A Python module implemented in Rust.
#[pymodule]
mod file {
    use pyo3::prelude::*;

    use crate::image_processors::{process_multiple_images};

    #[pyfunction]
    fn py_process_images(encoded_image_bytes: Vec<Vec<u8>>) -> PyResult<Vec<Vec<u8>>> {
        let x = process_multiple_images(&encoded_image_bytes)?;
        Ok(x)

    }

    /// Formats the sum of two numbers as string.
    #[pyfunction]
    fn sum_as_string(a: usize, b: usize) -> PyResult<String> {
        Ok((a + b).to_string())
    }
}
