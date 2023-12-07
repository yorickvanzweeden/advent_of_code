use std::cmp::Ordering;
use std::collections::HashMap;

#[derive(Debug)]
struct Record {
    hand: String,
    bid: i32,
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Debug)]
enum HandRank {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

fn evaluate_hand(hand: &str, treat_as_joker: bool) -> HandRank {
    let mut count_map = HashMap::new();
    let mut num_jokers = 0;

    for card in hand.chars() {
        if card == 'J' && treat_as_joker {
            num_jokers += 1;
        } else {
            let counter = count_map.entry(card).or_insert(0);
            *counter += 1;
        }
    }

    let mut counts: Vec<_> = count_map.values().cloned().collect();
    counts.sort_unstable();

    // Add jokers to the last count (i.e., the highest count) as they can represent any card.
    if treat_as_joker {
        let num_counts = counts.len();
        if counts.len() > 0 && num_jokers > 0 {
            counts[num_counts - 1] += num_jokers;
        } else if num_counts == 0 && num_jokers == 5 {
            counts.insert(0, 5);
        }
    }

    match counts.as_slice() {
        [five] if *five == 5 => HandRank::FiveOfAKind,
        [one, four] if *one == 1 && *four == 4 => HandRank::FourOfAKind,
        [two, three] if *two == 2 && *three == 3 => HandRank::FullHouse,
        [_, _, three] if *three == 3 => HandRank::ThreeOfAKind,
        [one, _, two] if *one == 1 && *two == 2 => HandRank::TwoPair,
        [_, _, _, two] if *two == 2 => HandRank::OnePair,
        _ => HandRank::HighCard,
    }
}

fn evaluate_cards(hand: &str, treat_as_joker: bool) -> Vec<i32> {
    let mut cards_rank = HashMap::new();

    cards_rank.insert('T', 10);
    if treat_as_joker {
        cards_rank.insert('J', 1);
    } else {
        cards_rank.insert('J', 11);
    }
    cards_rank.insert('Q', 12);
    cards_rank.insert('K', 13);
    cards_rank.insert('A', 14);

    hand.chars()
        .map(|card| match cards_rank.get(&card) {
            Some(rank) => *rank,
            None => card.to_digit(10).unwrap_or(0) as i32,
        })
        .collect::<Vec<i32>>()
}

fn compare_hands(hand1: &str, hand2: &str, treat_as_joker: bool) -> Ordering {
    let hand_rank1 = evaluate_hand(hand1, treat_as_joker);
    let hand_rank2 = evaluate_hand(hand2, treat_as_joker);

    if hand_rank1 != hand_rank2 {
        hand_rank1.cmp(&hand_rank2)
    } else {
        let strength1 = evaluate_cards(hand1, treat_as_joker);
        let strength2 = evaluate_cards(hand2, treat_as_joker);
        strength1
            .iter()
            .zip(strength2.iter())
            .find(|&(a, b)| a != b)
            .map(|(a, b)| a.cmp(&b))
            .unwrap_or(Ordering::Equal)
    }
}

fn parse_input(input: &str) -> Vec<Record> {
    let lines: Vec<&str> = input.split("\n").collect();
    lines
        .into_iter()
        .map(|line| {
            let parts: Vec<&str> = line.split_whitespace().collect();
            Record {
                hand: parts[0].to_string(),
                bid: parts[1].parse().unwrap(),
            }
        })
        .collect()
}

fn main_loop(input: &str, treat_as_joker: bool) -> i32 {
    let mut records = parse_input(input);
    records.sort_by(|a, b| compare_hands(&a.hand, &b.hand, treat_as_joker));

    records
        .iter()
        .enumerate()
        .map(|(index, record)| (index + 1) as i32 * record.bid)
        .sum()
}

#[aoc(day7, part1)]
pub fn part1(input: &str) -> i32 {
    main_loop(&input, false)
}

#[aoc(day7, part2)]
pub fn part2(input: &str) -> i32 {
    main_loop(&input, true)
}

#[cfg(test)]
mod tests {
    use super::{part1, part2};

    #[test]
    fn basics() {
        let input = "32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483";
        assert_eq!(part1(input), 6440);
        assert_eq!(part2(input), 5905);
    }
}
