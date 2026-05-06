import iris_bridge_py as ip
from pathlib import Path
import time

def benchmark_processor():
    # 1. Setup paths
    image_dir = Path("./src/images")
    image_paths = list(image_dir.glob("*.jpg")) + list(image_dir.glob("*.png"))

    if not image_paths:
        print(f"No images found in {image_dir}")
        return

    # 2. Pre-load images into memory (to exclude disk I/O from the Rust benchmark)
    encoded_images = [p.read_bytes() for p in image_paths]
    num_images = len(encoded_images)
    total_bytes = sum(len(b) for b in encoded_images) / (1024 * 1024)

    print(f"--- Benchmarking {num_images} images ({total_bytes:.2f} MB) ---")

    # 3. Start Timer
    start_time = time.perf_counter()

    try:
        results = ip.py_parallel_process_images(encoded_images)
        
        # 4. End Timer
        end_time = time.perf_counter()
        duration = end_time - start_time

        # 5. Calculate Metrics
        print(f"Successfully processed: {len(results)} images")
        print(f"Total Time:             {duration:.4f} seconds")
        print(f"Avg Time per Image:    {(duration / num_images) * 1000:.2f} ms")
        print(f"Throughput:            {num_images / duration:.2f} images/sec")

        if results:
            print(f"Output Tensor Shape:   {results[0].shape}")

    except ValueError as e:
        print(f"Rust side error: {e}")

if __name__ == "__main__":
    benchmark_processor()