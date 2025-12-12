advent_of_code::solution!(12);

pub fn part_one(input: &str) -> Option<usize> {
    let result = input
        .rsplit("\n\n")
        .next()
        .unwrap()
        .lines()
        .filter(|x| {
            let (dims, rest) = x.split_once(": ").unwrap();
            let (x, y) = dims.split_once("x").unwrap();
            let x = x.parse::<usize>().unwrap();
            let y = y.parse::<usize>().unwrap();
            let shape_max_area = rest
                .split_whitespace()
                .map(|x| x.parse::<usize>().unwrap())
                .sum::<usize>()
                * 9;
            let region_area = x * y;

            // the input is constructed in such a way that shapes don't need to overlap in any way for regions that can fit all their
            // shapes. therefore, checking whether the available area is larger than or equal to the most naive tiling is good enough
            region_area >= shape_max_area
        })
        .count();

    Some(result)
}

pub fn part_two(input: &str) -> Option<u64> {
    None
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
