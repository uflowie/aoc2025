advent_of_code::solution!(2);

pub fn part_one(input: &str) -> Option<u64> {
    let is_invalid = |x: u64| {
        let x_as_str = x.to_string();
        x_as_str[0..x_as_str.len() / 2] == x_as_str[x_as_str.len() / 2..]
    };

    Some(solve(input, is_invalid))
}

pub fn part_two(input: &str) -> Option<u64> {
    let is_invalid = |x: u64| {
        let x_as_str = x.to_string();
        (0..x_as_str.len() / 2)
            .any(|i| x_as_str == format!("{}{}", &x_as_str[i + 1..], &x_as_str[..i + 1]))
    };

    Some(solve(input, is_invalid))
}

fn solve(input: &str, is_invalid: fn(u64) -> bool) -> u64 {
    input
        .split(",")
        .map(|x| x.split_once("-").unwrap())
        .map(|(l, r)| l.parse::<u64>().unwrap()..=r.parse::<u64>().unwrap())
        .flatten()
        .filter(|x| is_invalid(*x))
        .sum()
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
