import pixelart
import numpy as np
import cv2
import time
from PIL import Image


def get_image(upload):
    img = Image.open(upload)
    img_array = np.array(img)
    return img_array


def get_color(img):  # 後で消す
    # 画像の幅と高さを取得
    height, width, _ = img.shape

    # RGB値を格納する変数を初期化
    rgb_values = np.zeros((width, height, 3), dtype=np.uint64)

    # 各ピクセルのRGB値を取得して格納
    for w in range(width):
        for h in range(height):
            rgb_values[w][h][0] = img[h][w][0]  # R
            rgb_values[w][h][1] = img[h][w][1]  # G
            rgb_values[w][h][2] = img[h][w][2]  # B
    return rgb_values


if __name__ == "__main__":
    img = get_image("../example.png")
    h, w, _ = img.shape
    aaa = time.time()
    x = get_color(img)
    print(time.time() - aaa)

    a = time.time()
    pixelart.convert(
        x, np.array([[0, 0, 0], [222, 0, 222], [222, 222, 222]], dtype=np.uint64)
    )
    print(time.time() - a)
