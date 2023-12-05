#[derive(Debug)]
struct Rule {
    source_start: i64,
    source_end: i64,
    dest: i64,
}

#[derive(Debug)]
struct Segment {
    name: String,
    rules: Vec<Rule>,
}

impl Segment {
    fn new(name: String, rules: Vec<Rule>) -> Self {
        Segment { name, rules }
    }

    fn process(&self, input: i64) -> i64 {
        for rule in &self.rules {
            if rule.source_start <= input && input < rule.source_end {
                return input - rule.source_start + rule.dest;
            }
        }
        input
    }
}

fn parse_input(input: &str) -> Vec<Segment> {
    let segments: Vec<&str> = input.trim().split("\n\n").collect();
    let mut segment_vec = Vec::new();

    for segment in &segments[1..] {
        let lines: Vec<&str> = segment.split("\n").collect();
        let name = lines[0].replace(" map:", "").to_string();
        let mut rules_vec = Vec::new();

        for line in &lines[1..] {
            let nums: Vec<i64> = line.split_whitespace()
                .map(|s| s.parse().unwrap())
                .collect();

            let rule = Rule {
                source_start: nums[1],
                source_end: nums[1] + nums[2],
                dest: nums[0],
            };
            rules_vec.push(rule);
        }
        segment_vec.push(Segment::new(name, rules_vec));
    }
    segment_vec
}


#[aoc(day5, part1)]
pub fn part1(input: &str) -> i64 {
    let segments = parse_input(input);
    let seeds: Vec<i64> = input.split("\n")
        .next().unwrap()
        .split(":")
        .nth(1).unwrap()
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();

    seeds.iter().map(|&seed| {
        segments.iter().fold(seed, |output, segment| {
            segment.process(output)
        })
    }).min().unwrap()
}

#[aoc(day5, part2)]
pub fn part2(input: &str) -> i64 {
    let segments = parse_input(input);

    let seeds: Vec<i64> = input.split("\n")
        .next().unwrap()
        .split(":")
        .nth(1).unwrap()
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();

    let seeds_iter = seeds.chunks(2)
        .flat_map(|chunk| chunk[0]..chunk[0]+chunk[1]);

    seeds_iter.map(|seed| {
        segments.iter().fold(seed, |output, segment| {
            segment.process(output)
        })
    }).min().unwrap()
}

#[cfg(test)]
mod tests {
    use super::{part1, part2};

    #[test]
    fn basics() {
        let input = "seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4";
        assert_eq!(part1(input), 35);
        assert_eq!(part2(input), 46);
    }
}