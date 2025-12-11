use std::collections::HashMap;

advent_of_code::solution!(11);

pub fn part_one(input: &str) -> Option<u64> {
    let connections = get_connections(input);

    let you_to_out = paths_from_to("you", "out", &connections, &mut HashMap::new());

    Some(you_to_out)
}

pub fn part_two(input: &str) -> Option<u64> {
    let connections = get_connections(input);

    // only one of these will be > 0, otherwise we would have a cycle and infinitely many paths
    let dac_to_fft = paths_from_to("dac", "fft", &connections, &mut HashMap::new());
    let fft_to_dac = paths_from_to("fft", "dac", &connections, &mut HashMap::new());

    let (first_checkpoint, first_to_second, second_checkpoint) = if dac_to_fft > 0 {
        ("dac", dac_to_fft, "fft")
    } else {
        ("fft", fft_to_dac, "dac")
    };

    let svr_to_first = paths_from_to("svr", first_checkpoint, &connections, &mut HashMap::new());
    let second_to_out = paths_from_to(second_checkpoint, "out", &connections, &mut HashMap::new());

    Some(svr_to_first * first_to_second * second_to_out)
}

fn get_connections(input: &str) -> HashMap<&str, Vec<&str>> {
    input
        .lines()
        .map(|x| {
            let (start, connected_to) = x.split_once(": ").unwrap();
            let connected_to: Vec<_> = connected_to.split_whitespace().collect();
            (start, connected_to)
        })
        .collect()
}

fn paths_from_to<'a>(
    start: &'a str,
    end: &str,
    connections: &HashMap<&'a str, Vec<&'a str>>,
    results: &mut HashMap<&'a str, u64>,
) -> u64 {
    if start == end {
        1
    } else if let Some(result) = results.get(start) {
        *result
    } else if let Some(next_nodes) = connections.get(start) {
        let result = next_nodes
            .iter()
            .map(|x| paths_from_to(x, end, connections, results))
            .sum::<u64>();
        results.insert(start, result);
        result
    } else {
        0
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
