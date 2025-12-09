use std::collections::HashSet;

advent_of_code::solution!(8);

type JunctionBox = (f64, f64, f64);

pub fn part_one(input: &str) -> Option<usize> {
    let boxes = input
        .lines()
        .map(|x| {
            let (x, rest) = x.split_once(",").unwrap();
            let (y, z) = rest.split_once(",").unwrap();
            (
                x.parse::<f64>().unwrap(),
                y.parse::<f64>().unwrap(),
                z.parse::<f64>().unwrap(),
            )
        })
        .collect::<Vec<_>>();

    let mut distances = vec![];

    for i in 0..boxes.len() {
        for j in i + 1..boxes.len() {
            distances.push((i, j, line_distance(&boxes[i], &boxes[j])))
        }
    }

    distances.sort_unstable_by(|a, b| a.2.partial_cmp(&b.2).unwrap());

    let mut circuits: Vec<HashSet<usize>> = vec![];

    for (i, j, _) in distances.iter().take(1000) {
        let i_circuit_idx = circuits.iter().position(|x| x.contains(i));
        let j_circuit_idx = circuits.iter().position(|x| x.contains(j));

        match (i_circuit_idx, j_circuit_idx) {
            (Some(i_circuit), Some(j_circuit)) if i_circuit != j_circuit => {
                let old = circuits.remove(j_circuit);
                circuits[i_circuit - ((j_circuit < i_circuit) as usize)].extend(old.iter());
            }
            (Some(i_circuit), None) => {
                circuits[i_circuit].insert(*j);
            }
            (None, Some(j_circuit)) => {
                circuits[j_circuit].insert(*i);
            }
            (None, None) => circuits.push(HashSet::from([*i, *j])),
            _ => {}
        }
    }

    circuits.sort_unstable_by_key(|x| x.len());

    let result = circuits.iter().rev().map(|x| x.len()).take(3).product();

    Some(result)
}

pub fn part_two(input: &str) -> Option<f64> {
    let boxes = input
        .lines()
        .map(|x| {
            let (x, rest) = x.split_once(",").unwrap();
            let (y, z) = rest.split_once(",").unwrap();
            (
                x.parse::<f64>().unwrap(),
                y.parse::<f64>().unwrap(),
                z.parse::<f64>().unwrap(),
            )
        })
        .collect::<Vec<_>>();

    let mut distances = vec![];

    for i in 0..boxes.len() {
        for j in i + 1..boxes.len() {
            distances.push((i, j, line_distance(&boxes[i], &boxes[j])))
        }
    }

    distances.sort_unstable_by(|a, b| a.2.partial_cmp(&b.2).unwrap());

    let mut circuits: Vec<HashSet<usize>> = vec![];

    for (i, j, _) in &distances {
        let i_circuit_idx = circuits.iter().position(|x| x.contains(i));
        let j_circuit_idx = circuits.iter().position(|x| x.contains(j));

        match (i_circuit_idx, j_circuit_idx) {
            (Some(i_circuit), Some(j_circuit)) if i_circuit != j_circuit => {
                let old = circuits.remove(j_circuit);
                circuits[i_circuit - ((j_circuit < i_circuit) as usize)].extend(old.iter());
            }
            (Some(i_circuit), None) => {
                circuits[i_circuit].insert(*j);
            }
            (None, Some(j_circuit)) => {
                circuits[j_circuit].insert(*i);
            }
            (None, None) => circuits.push(HashSet::from([*i, *j])),
            _ => {}
        }

        if circuits[0].len() == boxes.len() {
            return Some(boxes[*i].0 * boxes[*j].0);
        }
    }

    panic!()
}

fn line_distance(a: &JunctionBox, b: &JunctionBox) -> f64 {
    let dx = b.0 - a.0;
    let dy = b.1 - a.1;
    let dz = b.2 - a.2;

    (dx * dx + dy * dy + dz * dz).sqrt()
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
