use std::collections::{HashMap, HashSet};

#[aoc(day4, part1)]
pub fn part1(input: &str) -> i32 {
    let lines = input.lines();
    let mut sum = 0;
    for line in lines {
        let parts: Vec<&str> = line.split(": ").collect();
        let line_parts: Vec<&str> = parts[1].split(" | ").collect();
        let winning = line_parts[0].split_whitespace().map(|x| x.parse().unwrap()).collect::<Vec<i32>>();
        let tickets = line_parts[1].split_whitespace().map(|x| x.parse().unwrap()).collect::<Vec<i32>>();

        let winning: HashSet<i32> = winning.into_iter().collect();
        let tickets: HashSet<i32> = tickets.into_iter().collect();

        let intersection: HashSet<_> = winning.intersection(&tickets).collect();

        if !intersection.is_empty() {
            sum += 2u32.pow(intersection.len() as u32) / 2;
        }
    }

    sum.try_into().unwrap()
}

#[aoc(day4, part2)]
pub fn part2(input: &str) -> i32 {
    let lines = input.lines();
    let mut map: HashMap<i32, i32> = HashMap::new();

    for (index, line) in lines.enumerate() {
        let parts: Vec<&str> = line.split(": ").collect();
        let line_parts: Vec<&str> = parts[1].split(" | ").collect();
        let winning = line_parts[0].split_whitespace().map(|x| x.parse().unwrap()).collect::<Vec<i32>>();
        let tickets = line_parts[1].split_whitespace().map(|x| x.parse().unwrap()).collect::<Vec<i32>>();

        let winning: HashSet<i32> = winning.into_iter().collect();
        let tickets: HashSet<i32> = tickets.into_iter().collect();

        let intersection: HashSet<_> = winning.intersection(&tickets).collect();

        let current_card_number = index + 1;
        // Add current card to multiplier
        *map.entry(current_card_number as i32).or_insert(0) += 1;
        let multiplier = *map.entry(current_card_number as i32).or_insert(0);

        // Process winnings of current card; default(1) + winnings
        for i in 0..intersection.len() {
            let card_number = current_card_number + i + 1;
            *map.entry(card_number as i32).or_insert(0) += multiplier;
        }

    }

    map.values().sum()
}

#[cfg(test)]
mod tests {
    use super::{part1, part2};

    // (()) and ()() both result in floor 0.
    #[test]
    fn sample1() {
        assert_eq!(part1("Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11"), 13);
    }

    // ) causes him to enter the basement at character position 1.
    #[test]
    fn sample2() {
        assert_eq!(part2("Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11"), 30);
    }
}