advent_of_code::solution!(6);

pub fn part_one(input: &str) -> Option<u64> {
    let numbers = input
        .lines()
        .rev()
        .skip(1)
        .map(|line| {
            line.split_ascii_whitespace()
                .map(|x| x.parse::<u64>().unwrap())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let operations = input
        .lines()
        .last()
        .unwrap()
        .split_ascii_whitespace()
        .collect::<Vec<_>>();

    let mut result = 0;

    for i in 0..numbers[0].len() {
        let column = numbers.iter().map(|x| x[i]);
        result += match operations[i] {
            "+" => column.sum::<u64>(),
            _ => column.product(),
        };
    }

    Some(result)
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut numbers = input
        .lines()
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    numbers.pop(); // remove last line (operators)

    let operators = input.lines().last().unwrap();

    let mut lengths = operators
        .split(['+', '*'])
        .filter(|x| !x.is_empty())
        .map(|x| x.len())
        .collect::<Vec<_>>();

    let l = lengths.len();

    lengths[l - 1] += 1; // mild hack: last operator doesnt have the extra space that separates columns, we add it here to keep the calculation correct

    let operators = operators.split_ascii_whitespace().collect::<Vec<_>>();

    let mut offset = 0;
    let mut result = 0;

    for i in 0..lengths.len() {
        let curr_len = lengths[i];
        let operator = operators[i];

        let curr_numbers = (0..curr_len).map(|j| {
            numbers
                .iter()
                .map(|x| x[offset + j])
                .filter(|x| !x.is_whitespace())
                .collect::<String>()
                .parse::<u64>()
                .unwrap()
        });

        match operator {
            "+" => result += curr_numbers.sum::<u64>(),
            _ => result += curr_numbers.product::<u64>(),
        };

        offset += curr_len + 1;
    }

    Some(result)
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
