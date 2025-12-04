use std::collections::HashMap;

use advent_of_code::{get_all_neighbor_values, indexed_chars};

advent_of_code::solution!(4);

pub fn part_one(input: &str) -> Option<usize> {
    let grid = indexed_chars(input);

    let removable_rolls = get_accessable_rolls(&grid).count();

    Some(removable_rolls)
}

pub fn part_two(input: &str) -> Option<usize> {
    let mut grid = indexed_chars(input);

    let total_rolls_removed = std::iter::from_fn(|| {
        let rolls: Vec<_> = get_accessable_rolls(&grid).collect();
        if rolls.is_empty() {
            return None;
        }

        for roll in &rolls {
            grid.remove(roll);
        }
        Some(rolls.len())
    })
    .sum();

    Some(total_rolls_removed)
}

fn get_accessable_rolls(grid: &HashMap<(i32, i32), char>) -> impl Iterator<Item = (i32, i32)> {
    grid.iter()
        .filter(|e| is_roll(*e.1))
        .filter(move |e| {
            get_all_neighbor_values(&grid, *e.0)
                .filter(|n| n.is_some_and(|x| is_roll(*x)))
                .count()
                < 4
        })
        .map(|e| *e.0)
}

fn is_roll(ch: char) -> bool {
    ch == '@'
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
