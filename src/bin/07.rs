use std::collections::{HashMap, HashSet};

use advent_of_code::indexed_chars;

advent_of_code::solution!(7);

type Beam = (i32, i32);

pub fn part_one(input: &str) -> Option<usize> {
    let grid = indexed_chars(input);

    let starting_position = grid
        .iter()
        .find(|e| *e.1 == 'S')
        .map(|((x, y), _)| (x + 1, *y))
        .unwrap();

    let mut beams = HashSet::from([starting_position]);

    let mut splits = 0;

    while !beams.is_empty() {
        beams = beams.into_iter().map(|(x, y)| (x + 1, y)).collect();

        let split_beams = beams
            .iter()
            .filter(|b| grid.get(b).is_some_and(|x| *x == '^'))
            .map(|(x, y)| [(*x, y - 1), (*x, y + 1)])
            .flatten()
            .collect::<Vec<_>>();

        splits += split_beams.len() / 2;

        beams.extend(split_beams);

        beams.retain(|b| grid.get(&b).is_some_and(|x| *x != '^'));
    }

    Some(splits)
}

pub fn part_two(input: &str) -> Option<u64> {
    let grid = indexed_chars(input);

    let starting_position = grid
        .iter()
        .find(|e| *e.1 == 'S')
        .map(|((x, y), _)| (x + 1, *y))
        .unwrap();

    let timelines = get_timelines(&grid, starting_position, &mut HashMap::new());

    Some(timelines)
}

fn get_timelines(
    grid: &HashMap<(i32, i32), char>,
    curr_pos: (i32, i32),
    results: &mut HashMap<(i32, i32), u64>,
) -> u64 {
    let (x, y) = curr_pos;

    if let Some(cached) = results.get(&curr_pos) {
        *cached
    } else {
        let result = match grid.get(&curr_pos) {
            None => 1,
            Some('^') => {
                get_timelines(grid, (x, y + 1), results) + get_timelines(grid, (x, y - 1), results)
            }
            _ => get_timelines(grid, (x + 1, y), results),
        };
        results.insert(curr_pos, result);
        result
    }
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
