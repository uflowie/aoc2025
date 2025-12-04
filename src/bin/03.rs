advent_of_code::solution!(3);

pub fn part_one(input: &str) -> Option<u64> {
    Some(solve(input, 2))
}

pub fn part_two(input: &str) -> Option<u64> {
    Some(solve(input, 12))
}

fn solve(input: &str, num_batteries: u64) -> u64 {
    let slots = num_batteries as usize;

    input
        .lines()
        .map(|bank| max_voltage(bank.as_bytes(), slots))
        .sum()
}

fn max_voltage(bank: &[u8], mut slots: usize) -> u64 {
    let mut start = 0;
    let mut value = 0;

    while slots > 0 {
        let end = bank.len() - slots + 1;
        let (idx, digit) = bank[start..end]
            .iter()
            .copied()
            .enumerate()
            .max_by_key(|&(_, b)| b)
            .unwrap();

        value = value * 10 + (digit - b'0') as u64;
        start += idx + 1;
        slots -= 1;
    }

    value
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
