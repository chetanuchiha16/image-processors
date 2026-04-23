# 🖼️ Image Processors

A high-performance image processing library written in Rust, featuring both sequential and parallel batch processing capabilities. Built with [Rayon](https://github.com/rayon-rs/rayon) for data-parallel execution and [tracing](https://github.com/tokio-rs/tracing) for structured, span-level diagnostics.

## Features

- **Single image processing** — Decode, resize (224×224 Lanczos3), and re-encode individual images.
- **Batch processing** — Process a collection of images sequentially with iterator chaining.
- **Parallel processing** — Leverage all available CPU cores via Rayon's `par_iter` for significant throughput gains on large batches.
- **Format auto-detection** — Automatically infers image format (JPEG, PNG, etc.) from raw bytes.
- **Instrumented tracing** — Batch and parallel functions are annotated with `#[instrument]` spans, so you get structured timing data out of the box.

## Project Structure

```
image-processors/
├── Cargo.toml
├── src/
│   ├── main.rs                 # Entry point — orchestrates the full pipeline
│   ├── get_image_bytes.rs      # File I/O — discovers image paths and reads bytes
│   └── image_processors.rs     # Core processing — resize single, batch, and parallel
└── README.md
```

## Prerequisites

- **Rust** — 2024 edition (`rustup` recommended: https://rustup.rs)
- A directory of `.jpg` / `.png` images placed at `./src/images/`

## Getting Started

### 1. Clone the repository

```bash
git clone https://github.com/Hinaverse/image-processors.git
cd image-processors
```

### 2. Add images

Place your `.jpg` and/or `.png` files in `./src/images/`:

```bash
mkdir -p src/images
cp /path/to/your/photos/*.jpg src/images/
```

### 3. Build and run

```bash
cargo run
```

You should see tracing output showing the instrumented span durations for the batch and parallel processing steps:

```
The current directory is: "/path/to/image-processors"
INFO process_multiple_images{}: close time.busy=XXms time.idle=XXµs
INFO parallel_process_images{}: close time.busy=XXms time.idle=XXµs
processed N images
```

## API Reference

### `get_image_bytes` module

| Function | Signature | Description |
|---|---|---|
| `get_image_paths` | `(image_path: &str) -> Result<Vec<PathBuf>, Error>` | Scans a directory and returns paths to all `.jpg` and `.png` files. |
| `get_encoded_image_bytes` | `(paths: &[PathBuf]) -> Result<Vec<Vec<u8>>, Error>` | Reads each file into a `Vec<u8>` of raw encoded bytes. |

### `image_processors` module

| Function | Signature | Description |
|---|---|---|
| `process_single_image` | `(encoded_image_bytes: &[u8]) -> Result<Vec<u8>, ImageError>` | Decodes an image from bytes, resizes to **224×224** using Lanczos3 interpolation, and re-encodes to the original format. |
| `process_multiple_images` | `<T: AsRef<[u8]> + Sync>(encoded_image_bytes: &[T]) -> Result<Vec<Vec<u8>>, ImageError>` | Sequentially processes a batch of images. Instrumented with `tracing`. |
| `parallel_process_images` | `<T: AsRef<[u8]> + Sync>(encoded_image_bytes: &[T]) -> Result<Vec<Vec<u8>>, ImageError>` | Processes a batch of images **in parallel** using Rayon. Instrumented with `tracing`. |

## Dependencies

| Crate | Version | Purpose |
|---|---|---|
| [`image`](https://crates.io/crates/image) | 0.25.10 | Image decoding, encoding, and manipulation |
| [`rayon`](https://crates.io/crates/rayon) | 1.12.0 | Data-parallel iterators for multi-threaded processing |
| [`tracing`](https://crates.io/crates/tracing) | 0.1.44 | Structured, span-based diagnostics |
| [`tracing-subscriber`](https://crates.io/crates/tracing-subscriber) | 0.3.23 | Subscriber implementation with `env-filter` support |
| [`instrument`](https://crates.io/crates/instrument) | 0.2.0 | Procedural macro for `#[instrument]` annotations |

## Performance Notes

The parallel implementation (`parallel_process_images`) uses Rayon's work-stealing thread pool, which automatically scales to the number of available CPU cores. For I/O-bound workloads with many small images, the sequential version may perform comparably due to scheduling overhead — but for CPU-heavy batches (large images, expensive resizes), the parallel path provides near-linear speedup.


This project is part of the Hinaverse.
