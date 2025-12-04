use std::collections::HashMap;

pub mod template;

pub fn indexed_chars(input: &str) -> HashMap<(i32, i32), char> {
    indexed_chars_iter(input)
        .map(|(i, j, ch)| ((i, j), ch))
        .collect()
}

pub fn indexed_chars_iter(input: &str) -> impl Iterator<Item = (i32, i32, char)> + '_ {
    input.lines().enumerate().flat_map(|(i, line)| {
        line.chars()
            .enumerate()
            .map(move |(j, ch)| (i as i32, j as i32, ch))
    })
}

pub const HORIZONTAL_VERTICAL_DIRECTIONS: [(i32, i32); 4] = [(0, 1), (1, 0), (0, -1), (-1, 0)];

pub const DIAGONAL_DIRECTIONS: [(i32, i32); 4] = [(1, 1), (1, -1), (-1, 1), (-1, -1)];

pub fn get_row_column_neighbor_indices(idx: (i32, i32)) -> [(i32, i32); 4] {
    [
        add(idx, HORIZONTAL_VERTICAL_DIRECTIONS[0]),
        add(idx, HORIZONTAL_VERTICAL_DIRECTIONS[1]),
        add(idx, HORIZONTAL_VERTICAL_DIRECTIONS[2]),
        add(idx, HORIZONTAL_VERTICAL_DIRECTIONS[3]),
    ]
}

pub fn get_all_neighbor_indices(idx: (i32, i32)) -> [(i32, i32); 8] {
    [
        add(idx, HORIZONTAL_VERTICAL_DIRECTIONS[0]),
        add(idx, HORIZONTAL_VERTICAL_DIRECTIONS[1]),
        add(idx, HORIZONTAL_VERTICAL_DIRECTIONS[2]),
        add(idx, HORIZONTAL_VERTICAL_DIRECTIONS[3]),
        add(idx, DIAGONAL_DIRECTIONS[0]),
        add(idx, DIAGONAL_DIRECTIONS[1]),
        add(idx, DIAGONAL_DIRECTIONS[2]),
        add(idx, DIAGONAL_DIRECTIONS[3]),
    ]
}

pub fn get_all_neighbor_values(
    grid: &HashMap<(i32, i32), char>,
    idx: (i32, i32),
) -> impl Iterator<Item = Option<&char>> {
    get_all_neighbor_indices(idx)
        .into_iter()
        .map(|x| grid.get(&x))
}

pub fn add(left: (i32, i32), right: (i32, i32)) -> (i32, i32) {
    (left.0 + right.0, left.1 + right.1)
}
