use std::collections::{HashSet, VecDeque};

advent_of_code::solution!(10);

pub fn part_one(input: &str) -> Option<u32> {
    let result = input
        .lines()
        .map(Machine::from)
        .map(|x| x.solve_lights())
        .sum();
    Some(result)
}

pub fn part_two(input: &str) -> Option<u32> {
    let result = input
        .lines()
        .map(Machine::from)
        .map(|x| x.solve_joltage())
        .sum();
    Some(result)
}

#[derive(Debug)]
struct Machine {
    target_lights: LightDiagram,
    target_joltage: JoltageRequirements,
    buttons: Vec<Vec<usize>>,
}

impl Machine {
    fn solve_lights(&self) -> u32 {
        let initial_lights = self.target_lights.initial_state();
        let mut seen = HashSet::from([initial_lights.clone()]);

        let mut queue = VecDeque::from([initial_lights]);
        let mut depth = 0;

        while !queue.is_empty() {
            if queue.iter().any(|l| *l == self.target_lights) {
                break;
            }
            depth += 1;
            let states: Vec<_> = queue.drain(..).collect();
            for state in states {
                let new_states: Vec<_> = self
                    .buttons
                    .iter()
                    .map(|b| state.press_button(b))
                    .filter(|x| !seen.contains(x))
                    .collect();
                seen.extend(new_states.clone());
                queue.extend(new_states);
            }
        }

        depth
    }


    fn solve_joltage(&self) -> u32 {
        println!("solving joltage");
        let initial_joltage = self.target_joltage.initial_state();
        let mut seen = HashSet::from([initial_joltage.clone()]);

        let mut queue = VecDeque::from([initial_joltage]);
        let mut depth = 0;

        while !queue.is_empty() {
            if queue.iter().any(|l| *l == self.target_joltage) {
                break;
            }
            depth += 1;
            let states: Vec<_> = queue.drain(..).collect();
            for state in states {
                let new_states: Vec<_> = self
                    .buttons
                    .iter()
                    .map(|b| state.press_button(b))
                    .filter(|x| !seen.contains(x))
                    .filter(|x| x.target_is_still_reachable(&self.target_joltage))
                    .collect();
                seen.extend(new_states.clone());
                queue.extend(new_states);
            }
        }

        depth
    }
}

impl From<&str> for Machine {
    fn from(value: &str) -> Self {
        let parts: Vec<_> = value.split(" ").collect();

        let target_lights = {
            let mut l = vec![];
            for ch in parts[0].chars() {
                if ch == '.' {
                    l.push(false);
                } else if ch == '#' {
                    l.push(true)
                }
            }
            LightDiagram(l)
        };

        let buttons = parts[1..]
            .iter()
            .filter_map(|x| {
                if x.starts_with("{") {
                    None
                } else {
                    Some(
                        x[1..x.len() - 1]
                            .split(",")
                            .map(|x| x.parse::<usize>().unwrap())
                            .collect(),
                    )
                }
            })
            .collect();

        let last = parts.last().unwrap();
        let joltage_requirements = last[1..last.len() - 1]
            .split(",")
            .map(|x| x.parse::<usize>().unwrap())
            .collect();
        let target_joltage = JoltageRequirements(joltage_requirements);

        Self {
            target_lights,
            target_joltage,
            buttons,
        }
    }
}

#[derive(Hash, Eq, PartialEq, Clone, Debug)]
struct LightDiagram(Vec<bool>);

impl LightDiagram {
    fn press_button(&self, button: &[usize]) -> Self {
        let mut lights = self.0.clone();

        for i in button {
            lights[*i] = !lights[*i];
        }

        Self(lights)
    }

    fn initial_state(&self) -> Self {
        Self(vec![false; self.0.len()])
    }
}

#[derive(Hash, Eq, PartialEq, Clone, Debug)]
struct JoltageRequirements(Vec<usize>);

impl JoltageRequirements {
    fn press_button(&self, button: &[usize]) -> Self {
        let mut reqs = self.0.clone();

        for i in button {
            reqs[*i] += 1;
        }

        Self(reqs)
    }

    fn initial_state(&self) -> Self {
        Self(vec![0; self.0.len()])
    }

    fn target_is_still_reachable(&self, other: &Self) -> bool {
        self.0.iter().zip(other.0.iter()).all(|(a, b)| a <= b)
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
