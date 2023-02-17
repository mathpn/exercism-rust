use itertools::iproduct;
use std::iter::zip;

pub fn annotate(minefield: &[&str]) -> Vec<String> {
    let field: Vec<&[u8]> = minefield.iter().map(|s| s.as_bytes()).collect();
    let n_rows = field.len();
    let all_counts: Vec<Vec<u8>> = (0..n_rows)
        .map(|row| {
            let n_cols = field[row].len();
            (0..n_cols)
                .map(|col| {
                    let neighbors = get_neighbors(row as i8, col as i8, n_rows as i8, n_cols as i8);
                    return count_from_neighbors(neighbors, &field);
                })
                .collect()
        })
        .collect();
    return zip(all_counts, field)
        .map(|(counts, row)| {
            return zip(counts, row)
                .map(|(count, char)| {
                    if char == &b'*' {
                        "*".to_string()
                    } else if count > 0 {
                        count.to_string()
                    } else {
                        " ".to_string()
                    }
                })
                .collect::<String>();
        })
        .collect();
}

fn get_neighbors(row: i8, col: i8, n_rows: i8, n_cols: i8) -> Vec<(i8, i8)> {
    let pairs: Vec<(i8, i8)> = iproduct!(row - 1..=row + 1, col - 1..=col + 1)
        .filter(|item| item.0 >= 0 && item.1 >= 0 && item.0 < n_rows && item.1 < n_cols)
        .collect();
    return pairs;
}

fn count_from_neighbors(neighbors: Vec<(i8, i8)>, field: &Vec<&[u8]>) -> u8 {
    return neighbors
        .iter()
        .map(|pos| {
            if field[pos.0 as usize][pos.1 as usize] == 42 {
                1
            } else {
                0
            }
        })
        .sum();
}
