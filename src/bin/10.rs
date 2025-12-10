use std::collections::{HashSet, VecDeque};

use z3::{Optimize, ast::Int};

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
        let optimizer = Optimize::new();

        let presses_per_button: Vec<_> = self
            .buttons
            .iter()
            .enumerate()
            .map(|(i, _)| Int::fresh_const(&format!("presses_{}", i)))
            .collect();

        let mut joltages: Vec<_> = self
            .target_joltage
            .0
            .iter()
            .map(|_| Int::from_i64(0))
            .collect();

        for (i, button) in self.buttons.iter().enumerate() {
            for j in button {
                joltages[*j] = &joltages[*j] + &presses_per_button[i];
            }
        }

        for presses in &presses_per_button {
            optimizer.assert(&presses.ge(0));
        }

        for (i, joltage) in joltages.iter().enumerate() {
            optimizer.assert(&joltage.eq(self.target_joltage.0[i] as u32));
        }

        let sum_presses = Int::add(&presses_per_button);

        optimizer.minimize(&sum_presses);

        optimizer.check(&[]);
        let model = optimizer.get_model().unwrap();
        model.eval(&sum_presses, true).unwrap().as_u64().unwrap() as u32
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

#[derive(Debug)]
struct JoltageRequirements(Vec<usize>);

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
