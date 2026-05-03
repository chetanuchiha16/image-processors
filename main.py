import image_processors_py as ip
import numpy as np

# 1. Prepare some "encoded" image bytes
# In a real scenario, you'd open a file: open("img.jpg", "rb").read()
fake_image = b"not-a-real-image-but-valid-bytes"
image_list = [fake_image, fake_image]

try:
    # 2. Call your Rust parallel processor
    # This returns a list of NumPy arrays
    processed_images = ip.py_parallel_process_images(image_list)

    for i, img in enumerate(processed_images):
        print(f"Image {i} shape: {img.shape}, dtype: {img.dtype}")
        # 'img' is a standard numpy.ndarray, so you can use it normally
        print(f"Mean pixel value: {np.mean(img)}")

except ValueError as e:
    # This catches the 'ProcessorError' you converted in Rust
    print(f"Rust side error: {e}")