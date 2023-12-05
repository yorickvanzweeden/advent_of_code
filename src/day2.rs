use std::cmp::max;
use std::collections::HashMap;

#[aoc(day2, part1)]
pub fn part1(input: &str) -> i32 {
    let lines = input.lines();
    let mut sum = 0;
    for line in lines {
        let parts = line.split(':').collect::<Vec<&str>>();
        let first = parts[0];
        let game_number = first.split(" ").collect::<Vec<&str>>();
        let game_number: i32 = game_number[1].parse().unwrap();
        let mut valid: bool = true;

        for game in parts[1].split(';').collect::<Vec<&str>>() {
            let mut color_count = HashMap::new();
            color_count.insert("blue", 0);
            color_count.insert("red", 0);
            color_count.insert("green", 0);
            for move_ in game.split(',').collect::<Vec<&str>>() {
                let parts = move_.trim().split(' ').collect::<Vec<&str>>();
                let num: i32 = parts[0].parse().unwrap();
                let item = parts[1];
                color_count.insert(item, color_count.get(item).unwrap() + num);
            }
            if let Some(&value) = color_count.get("red") {
                if value > 12 {
                    valid = false;
                }
            }
            // Retrieve the item and check if it's > 13
            if let Some(&value) = color_count.get("green") {
                if value > 13 {
                    valid = false;
                }
            }
            // Retrieve the item and check if it's > 13
            if let Some(&value) = color_count.get("blue") {
                if value > 14 {
                    valid = false;
                }
            }
        }

        if valid == true {
            sum += game_number;
        }
    }
    sum
}

#[aoc(day2, part2)]
pub fn part2(input: &str) -> i32 {
    let lines = input.lines();
    let mut sum = 0;
    for line in lines {
        let parts = line.split(':').collect::<Vec<&str>>();
        let mut color_count = HashMap::new();
        color_count.insert("blue", 0);
        color_count.insert("red", 0);
        color_count.insert("green", 0);

        for game in parts[1].split(';').collect::<Vec<&str>>() {
            for move_ in game.split(',').collect::<Vec<&str>>() {
                let parts = move_.trim().split(' ').collect::<Vec<&str>>();
                let num: i32 = parts[0].parse().unwrap();
                let item = parts[1];
                color_count.insert(item, max(*color_count.get(item).unwrap(), num));
            }
        }

        sum += color_count.values().fold(1, |acc, &val| acc * val);
    }
    sum
}

#[cfg(test)]
mod tests {
    use super::{part1, part2};

    // (()) and ()() both result in floor 0.
    #[test]
    fn sample1() {
        assert_eq!(
            part1(
                "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green"
            ),
            8
        );
    }

    // ) causes him to enter the basement at character position 1.
    #[test]
    fn sample2() {
        assert_eq!(
            part2(
                "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green"
            ),
            2286
        );
    }
}
