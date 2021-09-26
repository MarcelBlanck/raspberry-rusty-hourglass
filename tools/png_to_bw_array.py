import sys
import imageio
import numpy as np


if len(sys.argv) != 3:
    print("Usage: python3 png_to_bw_array.py [threshold 0-255] [path to greyscale png file]")
    quit()

threshold = int(sys.argv[1])
image_data = imageio.imread(sys.argv[2])

with np.nditer(image_data, op_flags=['readwrite']) as it:
   for x in it:
       x[...] = 1 if x > threshold else 0

print("pub const PIXMAP: Pixmap = Pixmap {")
print("    data: [[", end = '')
num_rows, num_cols = image_data.shape
for y in range(0, num_rows):
    for x in range(0, num_cols):
        if x != num_cols - 1:
            print("{},".format(image_data[y,x]), end = '')
        else:
            print("{}".format(image_data[y,x]), end = '')
    if y != num_rows-1:
        print("],")
        print("    [", end = '')
    else:
        print("]", end = '')
print("]")
print("};")
