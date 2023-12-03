use std::collections::HashMap;

pub fn find_indices(input: &str, needle: &str) -> Vec<usize> {
    let mut indices = Vec::new();
    let mut start = 0;
    while let Some(found) = input[start..].find(needle) {
        indices.push(start + found);
        start += found + 1;
    }
    indices
}

#[aoc(day1, part1)]
pub fn part1(input: &str) -> i32 {
    let mut sum = 0;
    for line in input.lines() {
        let digits: Vec<char> = line.chars().filter(|c|c.is_numeric()).collect();
        let number_str: String = format!("{}{}", digits.first().unwrap(), digits.last().unwrap());
        let number: i32 = number_str.parse::<i32>().unwrap();
        sum += number;
    }
    return sum;
}

#[aoc(day1, part2)]
pub fn part2(input: &str) -> i32 {
    let mut replacements = HashMap::new();
    replacements.insert("one", "1");
    replacements.insert("two", "2");
    replacements.insert("three", "3");
    replacements.insert("four", "4");
    replacements.insert("five", "5");
    replacements.insert("six", "6");
    replacements.insert("seven", "7");
    replacements.insert("eight", "8");
    replacements.insert("nine", "9");

    let mut index_map = HashMap::new();
    for (k, v) in replacements.iter() {
        for index in find_indices(input, k) {
            index_map.insert(index, v);
        }
    }

    let mut input = input.to_string();
    let mut sorted_indices: Vec<_> = index_map.keys().collect();
    sorted_indices.sort_by(|a, b| b.cmp(a));
    for index in sorted_indices {
        let word_end = index + index_map[index].len();
        input.replace_range(*index..word_end, index_map[index]);
    }
    return part1(&input);
}

#[cfg(test)]
mod tests {
    use super::{part1, part2};

    // (()) and ()() both result in floor 0.
    #[test]
    fn sample1() {
        assert_eq!(part1("1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet"), 142);
    }

    // ) causes him to enter the basement at character position 1.
    #[test]
    fn sample6() {
        assert_eq!(part2("two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen"), 281);
    }
}