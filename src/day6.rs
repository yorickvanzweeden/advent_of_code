#[derive(Debug)]
struct Record {
    time: i64,
    distance: i64,
}

fn parse_input(input: &str) -> Vec<Record> {
    let lines: Vec<&str> = input.trim().split("\n").collect();
    let times: Vec<i64> = lines[0]
        .replace("Time:", "")
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();
    let distances: Vec<i64> = lines[1]
        .replace("Distance:", "")
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();

    times
        .into_iter()
        .zip(distances.into_iter())
        .map(|(time, distance)| Record { time, distance })
        .collect()
}

#[aoc(day6, part1)]
pub fn part1(input: &str) -> i64 {
    let records = parse_input(input);

    records
        .iter()
        .map(|item| {
            let time_range = 0..item.time;
            let res = time_range
                .into_iter()
                .map(|hold| hold * (item.time - hold))
                .filter(|&t| t > item.distance)
                .count();
            res
        })
        .fold(1, |acc, num| acc * num) as i64
}

#[aoc(day6, part2)]
pub fn part2(input: &str) -> i64 {
    let input = input.replace(' ', "");
    part1(&input)
}

#[cfg(test)]
mod tests {
    use super::{part1, part2};

    #[test]
    fn basics() {
        let input = "Time:      7  15   30
Distance:  9  40  200";
        assert_eq!(part1(input), 288);
        assert_eq!(part2(input), 71503);
    }
}
