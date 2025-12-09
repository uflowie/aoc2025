use itertools::Itertools;

advent_of_code::solution!(9);

type Vertex = (i64, i64);
type Edge = (Vertex, Vertex);

pub fn part_one(input: &str) -> Option<i64> {
    let tiles = get_tiles(input);

    pairwise(&tiles).map(get_area).max()
}

pub fn part_two(input: &str) -> Option<i64> {
    let tiles = get_tiles(input);

    let (vertical_edges, horizontal_edges): (Vec<_>, Vec<_>) = tiles
        .iter()
        .copied()
        .zip(tiles.iter().copied().cycle().skip(1))
        .partition(|((x1, _), (x2, _))| x1 == x2);

    pairwise(&tiles)
        .filter(|&(a, b)| {
            rectangle_consists_of_red_and_green_tiles_only(a, b, &vertical_edges, &horizontal_edges)
        })
        .map(get_area)
        .max()
}

fn get_area(((x1, y1), (x2, y2)): (Vertex, Vertex)) -> i64 {
    let a = (x1 - x2).abs() + 1;
    let b = (y1 - y2).abs() + 1;
    a * b
}

fn get_tiles(input: &str) -> Vec<Vertex> {
    input
        .lines()
        .map(|x| x.split_once(",").unwrap())
        .map(|(x, y)| (x.parse::<i64>().unwrap(), y.parse::<i64>().unwrap()))
        .collect()
}

fn pairwise(tiles: &[Vertex]) -> impl Iterator<Item = (Vertex, Vertex)> + '_ {
    tiles.iter().copied().enumerate().flat_map(|(i, first)| {
        tiles
            .iter()
            .copied()
            .skip(i + 1)
            .map(move |second| (first, second))
    })
}

fn strictly_between(a: i64, b: i64, v: i64) -> bool {
    let (lo, hi) = if a < b { (a, b) } else { (b, a) };
    v > lo && v < hi
}

fn rectangle_consists_of_red_and_green_tiles_only(
    (x1, y1): Vertex,
    (x2, y2): Vertex,
    vertical_edges: &[Edge],
    horizontal_edges: &[Edge],
) -> bool {
    let rem_corners = [(x1, y2), (x2, y1)];

    if rem_corners
        .iter()
        .any(|p| !point_is_inside(*p, vertical_edges))
    {
        return false;
    }

    let horizontal_sides = [((x1, y1), rem_corners[1]), ((x2, y2), rem_corners[0])];
    let vertical_sides = [((x1, y1), rem_corners[0]), ((x2, y2), rem_corners[1])];

    horizontal_sides
        .iter()
        .cartesian_product(vertical_edges.iter())
        .all(|(((x1, y1), (x2, y2)), ((x3, y3), (x4, y4)))| {
            !strictly_between(*x1, *x2, *x3) || !strictly_between(*y3, *y4, *y1)
        })
        && vertical_sides
            .iter()
            .cartesian_product(horizontal_edges.iter())
            .all(|(((x1, y1), (x2, y2)), ((x3, y3), (x4, y4)))| {
                !strictly_between(*y1, *y2, *y3) || !strictly_between(*x3, *x4, *x1)
            })
}

fn point_is_inside((x, y): Vertex, vertical_edges: &[Edge]) -> bool {
    let mut inside = false;

    for ((_, y1), (_, y2)) in vertical_edges.iter().filter(|((x1, _), _)| *x1 > x) {
        let y_lo = *y1.min(y2);
        let y_hi = *y1.max(y2);

        if y >= y_lo && y < y_hi {
            inside = !inside;
        }
    }

    inside
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
