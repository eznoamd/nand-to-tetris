import math

SCALE = 256

for angle in range(91):
    value = round(math.sin(math.radians(angle)) * SCALE)
    print(f"let sinTable[{angle}] = {value};")