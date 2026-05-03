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
mod image_processors_py {
    use ndarray::Array3;
    use numpy::{IntoPyArray, PyArray3};
    use pyo3::{prelude::*, types::PyBytes};

    use crate::image_processors::{parallel_process_images, process_multiple_images};

    #[pyfunction]
    // We add the 'py: Python' argument so we can bind the new arrays to the Python GIL
    fn py_process_images<'py>(
        py: Python<'py>,
        encoded_image_bytes: Vec<Vec<u8>>,
    ) -> PyResult<Vec<Bound<'py, PyArray3<f32>>>> {
        // 1. Call your internal Rust logic
        let rust_arrays: Vec<Array3<f32>> = process_multiple_images(&encoded_image_bytes)?;

        // 2. Map each Rust ndarray into a Python-bound NumPy array
        let py_arrays: Vec<Bound<'py, PyArray3<f32>>> = rust_arrays
            .into_iter()
            .map(|arr| arr.into_pyarray(py))
            .collect();

        Ok(py_arrays)
    }

    #[pyfunction]
    fn py_parallel_process_images<'py>(
        py: Python<'py>,
        encoded_image_bytes: Vec<Bound<'py, PyBytes>>, // Accept list of bytes
    ) -> PyResult<Vec<Bound<'py, PyArray3<f32>>>> {
        let bytes: Vec<_> = encoded_image_bytes.iter().map(|a| a.as_bytes()).collect();
        let rust_arrays: Vec<Array3<f32>> = parallel_process_images(&bytes)?;

        let py_arrays: Vec<Bound<'py, PyArray3<f32>>> = rust_arrays
            .into_iter()
            .map(|arr| numpy::PyArray3::from_array(py, &arr))
            .collect();

        Ok(py_arrays)
    }
}
