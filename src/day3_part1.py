import numpy as np
from scipy.ndimage import label

with open('input/2023/day3.txt', 'r') as f:
    lines = f.readlines()

symbol_matrix = []
digit_matrix = []
actual_matrix = []

for line in lines:
    line = line.strip()
    for char in line:
        if char == '.':
            digit_matrix.append(False)
            symbol_matrix.append(False)
            actual_matrix.append('')
        elif char.isdigit():
            digit_matrix.append(True)
            symbol_matrix.append(False)
            actual_matrix.append(char)
        else:
            digit_matrix.append(False)
            symbol_matrix.append(True)
            actual_matrix.append('')

digit_matrix = np.array(digit_matrix).reshape((len(lines), len(lines[0].strip())))
symbol_matrix = np.array(symbol_matrix).reshape((len(lines), len(lines[0].strip())))
actual_matrix = np.array(actual_matrix).reshape((len(lines), len(lines[0].strip())))

# Add buffer of False values around the matrix
symbol_matrix = np.pad(symbol_matrix, 1, constant_values=False)

# Match symbol_matrix with digit_matrix
symbol_matrix_up = symbol_matrix[:-2, 1:-1]
symbol_matrix_down = symbol_matrix[2:, 1:-1]
symbol_matrix_left = symbol_matrix[1:-1, :-2]
symbol_matrix_right = symbol_matrix[1:-1, 2:]
symbol_matrix_up_right = symbol_matrix[:-2, 2:]
symbol_matrix_up_left = symbol_matrix[:-2, :-2]
symbol_matrix_down_right = symbol_matrix[2:, 2:]
symbol_matrix_down_left = symbol_matrix[2:, :-2]

# Create AND mask
symbol_matrix_up = np.logical_and(symbol_matrix_up, digit_matrix)
symbol_matrix_down = np.logical_and(symbol_matrix_down, digit_matrix)
symbol_matrix_left = np.logical_and(symbol_matrix_left, digit_matrix)
symbol_matrix_right = np.logical_and(symbol_matrix_right, digit_matrix)
symbol_matrix_up_right = np.logical_and(symbol_matrix_up_right, digit_matrix)
symbol_matrix_up_left = np.logical_and(symbol_matrix_up_left, digit_matrix)
symbol_matrix_down_right = np.logical_and(symbol_matrix_down_right, digit_matrix)
symbol_matrix_down_left = np.logical_and(symbol_matrix_down_left, digit_matrix)

# Create OR mask of all AND masks
digit_matrix_or = np.logical_or(symbol_matrix_up, symbol_matrix_down)
digit_matrix_or = np.logical_or(digit_matrix_or, symbol_matrix_left)
digit_matrix_or = np.logical_or(digit_matrix_or, symbol_matrix_right)
digit_matrix_or = np.logical_or(digit_matrix_or, symbol_matrix_up_right)
digit_matrix_or = np.logical_or(digit_matrix_or, symbol_matrix_up_left)
digit_matrix_or = np.logical_or(digit_matrix_or, symbol_matrix_down_right)
digit_matrix_or = np.logical_or(digit_matrix_or, symbol_matrix_down_left)

labeled_matrix, num_labels = label(digit_matrix)

# Do an AND with the digit_matrix_or and the labeled_matrix
total_sum = 0
for i in range(1, num_labels+1):
    group_positions = np.where(labeled_matrix == i)
    is_hit = any(digit_matrix_or[group_positions])
    if is_hit:
        total_sum += int("".join(actual_matrix[group_positions]))

print(total_sum)
