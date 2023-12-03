from collections import defaultdict

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
        if char == '*':
            digit_matrix.append(False)
            symbol_matrix.append(True)
            actual_matrix.append('')
        elif char.isdigit():
            digit_matrix.append(True)
            symbol_matrix.append(False)
            actual_matrix.append(char)
        else:
            digit_matrix.append(False)
            symbol_matrix.append(False)
            actual_matrix.append('')

actual_matrix = np.array(actual_matrix).reshape((len(lines), len(lines[0].strip())))
digit_matrix = np.array(digit_matrix).reshape((len(lines), len(lines[0].strip())))
group_matrix, num_labels = label(digit_matrix)

symbol_matrix = np.array(symbol_matrix).reshape((len(lines), len(lines[0].strip())))

# Add buffer of False values around the matrix
group_matrix = np.pad(group_matrix, 1, constant_values=0)

group_matrix_up = group_matrix[:-2, 1:-1]
group_matrix_down = group_matrix[2:, 1:-1]
group_matrix_left = group_matrix[1:-1, :-2]
group_matrix_right = group_matrix[1:-1, 2:]
group_matrix_up_right = group_matrix[:-2, 2:]
group_matrix_up_left = group_matrix[:-2, :-2]
group_matrix_down_right = group_matrix[2:, 2:]
group_matrix_down_left = group_matrix[2:, :-2]

# position - group dict
position_group_dict = defaultdict(set)

for shift in [group_matrix_up, group_matrix_down, group_matrix_left, group_matrix_right, group_matrix_up_right,
              group_matrix_up_left, group_matrix_down_right, group_matrix_down_left]:
    positions = np.where(np.logical_and(symbol_matrix, shift))
    positions = list(zip(*positions))
    for x, y in positions:
        position_group_dict[(x, y)].add(shift[x, y])

    #     position_group_dict[position] = shift[position]

group_pairs_to_multiply = set()
for key, values in position_group_dict.items():
    if len(values) == 2:
        group_pairs_to_multiply.add(tuple(values))

group_to_numbers = []
for group_number in range(1, num_labels + 1):
    group_positions = np.where(group_matrix[1:-1, 1:-1] == group_number)
    group_to_numbers.append(int("".join(actual_matrix[group_positions])))

total_sum = 0
for group1, group2 in group_pairs_to_multiply:
    total_sum += group_to_numbers[group1 - 1] * group_to_numbers[group2 - 1]

print(total_sum)
