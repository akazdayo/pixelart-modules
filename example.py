import pixelart_modules as pixelart
import numpy as np
import cv2
import time
from PIL import Image


def get_image(upload):
    img = Image.open(upload)
    img_array = np.array(img)
    return img_array


if __name__ == "__main__":
    img = get_image("./example.png")
    a = time.time()
    pixelart.convert(
        img, np.array([[0, 0, 0], [222, 0, 222], [222, 222, 222]], dtype=np.uint64)
    )
    print(time.time() - a)
