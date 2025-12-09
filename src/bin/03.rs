advent_of_code::solution!(3);

pub fn part_one(input: &str) -> Option<u64> {
    Some(solve(input, 2))
}

pub fn part_two(input: &str) -> Option<u64> {
    Some(solve(input, 12))
}

fn solve(input: &str, num_batteries: u64) -> u64 {
    let total_joltage = input
        .lines()
        .map(|mut bank| {
            let mut max_voltage = 0;

            for i in 0..num_batteries {
                let (ch, rem) = find_largest_digit_that_fits(bank, (num_batteries - i) as usize);
                bank = rem;
                max_voltage += (ch.to_digit(10).unwrap() as u64)
                    * 10_u64.pow((num_batteries - i - 1).try_into().unwrap())
            }

            max_voltage
        })
        .sum();

    total_joltage
}

fn find_largest_digit_that_fits(input: &str, digits_after: usize) -> (char, &str) {
    let (max_idx, max_ch) = input
        .bytes()
        .take(input.len() - digits_after + 1)
        .enumerate()
        .fold((0, b'0'), |(best_idx, best_ch), (idx, ch)| {
            if ch > best_ch {
                (idx, ch)
            } else {
                (best_idx, best_ch)
            }
        });

    (max_ch as char, &input[max_idx + 1..])
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
