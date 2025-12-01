advent_of_code::solution!(1);

pub fn part_one(input: &str) -> Option<u64> {
    let (_, password) = parse_rotations(input).fold((50, 0_u64), |(curr, pwd), rotation| {
        let curr = curr + rotation.direction.sign() * rotation.magnitude;
        let pwd = pwd + (curr % 100 == 0) as u64;
        (curr, pwd)
    });

    Some(password)
}

pub fn part_two(input: &str) -> Option<i32> {
    let (_, password) = parse_rotations(input).fold((50, 0), |(curr, pwd), rotation| {
        let mut pwd = pwd + rotation.magnitude / 100; // whole spins
        let remainder = rotation.magnitude % 100;

        let crosses_boundary = match rotation.direction {
            Direction::Left => curr != 0 && remainder >= curr,
            Direction::Right => curr != 0 && curr + remainder >= 100,
        };

        let curr = (curr + rotation.direction.sign() * remainder).rem_euclid(100);
        if crosses_boundary {
            pwd += 1;
        }

        (curr, pwd)
    });

    Some(password)
}

enum Direction {
    Left,
    Right,
}

impl Direction {
    fn sign(&self) -> i32 {
        match self {
            Direction::Left => -1,
            Direction::Right => 1,
        }
    }
}

struct Rotation {
    direction: Direction,
    magnitude: i32,
}

fn parse_rotations(input: &str) -> impl Iterator<Item = Rotation> + '_ {
    input.lines().filter(|line| !line.is_empty()).map(Rotation::from)
}

impl From<&str> for Rotation {
    fn from(value: &str) -> Self {
        let (direction, magnitude) = value.split_at(1);
        Self {
            direction: match direction {
                "L" => Direction::Left,
                "R" => Direction::Right,
                _ => unreachable!("direction is always L or R"),
            },
            magnitude: magnitude.parse().expect("magnitude should be a number"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(6));
    }
}
