advent_of_code::solution!(5);

pub fn part_one(input: &str) -> Option<usize> {
    let (ranges, ids) = input.split_once("\n\n").unwrap();

    let ranges = ranges
        .lines()
        .map(|line| {
            let (min, max) = line.split_once("-").unwrap();
            min.parse::<u64>().unwrap()..=max.parse::<u64>().unwrap()
        })
        .collect::<Vec<_>>();

    let fresh_ids = ids
        .lines()
        .map(|id| id.parse::<u64>().unwrap())
        .filter(|id| ranges.iter().any(|r| r.contains(id)))
        .count();

    Some(fresh_ids)
}

pub fn part_two(input: &str) -> Option<u64> {
    let (ranges, ids) = input.split_once("\n\n").unwrap();

    let mut ranges = ranges
        .lines()
        .map(|line| {
            let (min, max) = line.split_once("-").unwrap();
            min.parse::<u64>().unwrap()..=max.parse::<u64>().unwrap()
        })
        .collect::<Vec<_>>();

    ranges.sort_unstable_by_key(|r| *r.start());

    let mut sum = ranges[0].end() - ranges[0].start() + 1;

    let mut last_end = ranges[0].end();

    for i in 1..ranges.len() {
        let range = &ranges[i];

        if range.end() <= last_end {
            // range is fully included in previous range
            continue;
        } else if range.start() > last_end {
            // ranges don't overlap
            sum += range.end() - range.start() + 1;
            last_end = range.end();
        } else {
            // ranges overlap partially
            sum += range.end() - last_end;
            last_end = range.end();
        }
    }

    Some(sum)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
