from pathlib import Path


MAP_FILE = Path("map.txt")


lines = [
    line.strip()
    for line in MAP_FILE.read_text().splitlines()
    if line.strip()
]

width = len(lines[0])
height = len(lines)

player_start_x = 0
player_start_y = 0

print(f"let width = {width};")
print(f"let height = {height};")
print(f"let data = Array.new({width * height});")

index = 0

for y, line in enumerate(lines):
    for x, value in enumerate(line):

        if value == "2":
            player_start_x = x
            player_start_y = y
            value = "0"

        print(f"let data[{index}] = {value};")
        index += 1

print(f"let playerStartX = {player_start_x};")
print(f"let playerStartY = {player_start_y};")