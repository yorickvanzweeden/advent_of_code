fn diff(nums: &[i64]) -> Vec<i64> {
    return nums.windows(2).map(|w| w[1] - w[0]).collect();
}

#[aoc(day9, part1)]
pub fn part1(input: &str) -> i64 {
    input
        .lines()
        .map(|line| {
            let nums: Vec<i64> = line
                .split_whitespace()
                .map(|s| s.parse().unwrap())
                .collect();

            let mut diffs = nums;
            let mut total_diff = diffs[diffs.len() - 1];
            while diffs.len() > 2 {
                diffs = diff(&diffs);
                total_diff += diffs[diffs.len() - 1];
            }
            total_diff
        })
        .sum()
}

#[aoc(day9, part2)]
pub fn part2(input: &str) -> i64 {
    input
        .lines()
        .map(|line| {
            let nums: Vec<i64> = line
                .split_whitespace()
                .map(|s| s.parse().unwrap())
                .collect();

            let mut diffs = nums;
            let mut starts = vec![diffs[0]];
            while diffs.len() > 2 {
                diffs = diff(&diffs);
                starts.push(diffs[0]);
            }
            starts.into_iter().rev().fold(0, |acc, x| x - acc)
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::{part1, part2};

    #[test]
    fn part1_test() {
        let input = "0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45";
        assert_eq!(part1(input), 114);
    }

    #[test]
    fn part2_test() {
        let input = "0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45";
        assert_eq!(part2(input), 2);
    }
}
