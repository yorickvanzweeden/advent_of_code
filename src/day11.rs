use itertools::Itertools;

fn calc_min_distance(input: &str, size: i64) -> i64 {
    let map: Vec<Vec<_>> = input.lines().map(|line| line.chars().collect()).collect();
    let mut galaxies: Vec<(i64, i64)> = Vec::new();

    // Track rows and columns with no '#'
    let mut empty_rows: Vec<usize> = Vec::new();
    let mut empty_cols: Vec<usize> = Vec::new();

    for (y, row) in map.iter().enumerate() {
        let mut row_has_galaxy = false;
        for (x, char) in row.iter().enumerate() {
            if char == &'#' {
                galaxies.push((x as i64, y as i64));
                row_has_galaxy = true;
            }
        }
        if !row_has_galaxy {
            empty_rows.push(y);
        }
    }

    // Check for empty columns
    for x in 0..map[0].len() {
        if !map.iter().any(|row| row[x] == '#') {
            empty_cols.push(x);
        }
    }

    // Adjust coordinates
    for galaxy in &mut galaxies {
        let (x, y) = *galaxy;
        let new_x = x + size * (empty_cols.iter().filter(|&&col| col < x as usize).count() as i64);
        let new_y = y + size * (empty_rows.iter().filter(|&&row| row < y as usize).count() as i64);
        *galaxy = (new_x, new_y);
    }

    galaxies
        .iter()
        .combinations(2)
        .map(|pair| {
            let ((x1, y1), (x2, y2)) = (pair[0], pair[1]);
            (x2 - x1).abs() + (y2 - y1).abs()
        })
        .sum()
}

#[aoc(day11, part1)]
pub fn part1(input: &str) -> i64 {
    calc_min_distance(input, 2 - 1)
}

#[aoc(day11, part2)]
pub fn part2(input: &str) -> i64 {
    calc_min_distance(input, 1_000_000 - 1)
}

#[cfg(test)]
mod tests {
    use super::calc_min_distance;

    #[test]
    fn test() {
        let input = "...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#.....";
        assert_eq!(calc_min_distance(input, 2 - 1), 374);
        assert_eq!(calc_min_distance(input, 10 - 1), 1030);
        assert_eq!(calc_min_distance(input, 100 - 1), 8410);
    }
}
