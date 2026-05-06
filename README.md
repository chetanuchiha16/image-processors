# 🖼️ IrisBridge

A high-performance image processing library written in Rust, featuring both sequential and parallel batch processing capabilities. Built as a part of the **Hinaverse**, IrisBridge provides a seamless **Python bridge** via PyO3, allowing you to process images in Rust and receive them as **NumPy arrays** in Python.

## Features

- **Python Integration** — Seamlessly call high-performance Rust processing from Python using `iris_bridge_py`.
- **NumPy Support** — Returns images as normalized 3D ndarrays (H, W, C) ready for ML models.
- **Parallel Processing** — Leverage all available CPU cores via Rayon's `par_iter` for significant throughput gains on large batches.
- **Batch Processing** — Process a collection of images sequentially with efficient iterator chaining.
- **ML-Ready Output** — Automatically resizes to 224×224 (Lanczos3) and normalizes pixel values to [0, 1].
- **Format Auto-detection** — Automatically infers image format (JPEG, PNG, etc.) from raw bytes.
- **Instrumented Tracing** — Functions are annotated with `#[instrument]` spans for structured, span-level timing diagnostics out of the box.

## Project Structure

```
iris-bridge/
├── Cargo.toml
├── src/
│   ├── lib.rs                  # Python module entry point & library root
│   ├── main.rs                 # CLI entry point — orchestrates the full pipeline
│   ├── get_image_bytes.rs      # File I/O — discovers image paths and reads bytes
│   └── image_processors.rs     # Core processing — resize and ndarray conversion
└── README.md
```

## Prerequisites

- **Rust** — 2024 edition (`rustup` recommended)
- **Python 3.8+** (for Python bindings)
- **maturin** (optional, for building Python extension: `pip install maturin`)

## Getting Started (Rust CLI)

### 1. Build and Run
```bash
cargo run -- --path ./src/images
```

You will see tracing output showing performance metrics:
```
INFO parallel_process_images{}: close time.busy=120ms time.idle=50µs
processed 50 images
```

## Getting Started (Python)

### 1. Build the Extension
```bash
maturin develop
```

### 2. Usage in Python
```python
import iris_bridge_py
import numpy as np

# List of image bytes
images = [open("img1.jpg", "rb").read(), open("img2.png", "rb").read()]

# Process in parallel
# Returns a list of numpy arrays (shape: [224, 224, 3], dtype: float32)
batch = iris_bridge_py.py_parallel_process_images(images)

print(f"Processed {len(batch)} images. First image shape: {batch[0].shape}")
```

## API Reference (Rust)

### `get_image_bytes` module

| Function | Signature | Description |
|---|---|---|
| `get_image_paths` | `(path: &str) -> Result<Vec<PathBuf>, Error>` | Scans directory for `.jpg`, `.jpeg`, and `.png` files. |
| `get_encoded_image_bytes` | `(paths: &[PathBuf]) -> Result<Vec<Vec<u8>>, Error>` | Reads multiple files into memory as raw bytes. |

### `image_processors` module

| Function | Signature | Description |
|---|---|---|
| `process_single_image` | `(bytes: &[u8]) -> Result<Vec<u8>, Error>` | Decodes, resizes to 224×224, and re-encodes. |
| `process_single_image_nd_array` | `(bytes: &[u8]) -> Result<Array3<f32>, Error>` | Returns a normalized 224×224×3 ndarray. |
| `process_multiple_images` | `(bytes: &[T]) -> Result<Vec<Array3<f32>>, Error>` | Batch sequential processing into ndarrays. |
| `parallel_process_images` | `(bytes: &[T]) -> Result<Vec<Array3<f32>>, Error>` | Batch parallel processing into ndarrays via Rayon. |

## Dependencies

| Crate | Purpose |
|---|---|
| [`image`](https://crates.io/crates/image) | Image manipulation & decoding |
| [`pyo3`](https://crates.io/crates/pyo3) | Rust bindings for Python |
| [`numpy`](https://crates.io/crates/numpy) | Rust/Python NumPy integration |
| [`ndarray`](https://crates.io/crates/ndarray) | N-dimensional arrays for Rust |
| [`rayon`](https://crates.io/crates/rayon) | Data-parallelism |
| [`tracing`](https://crates.io/crates/tracing) | Performance diagnostics |

---
Part of the **Hinaverse**.
