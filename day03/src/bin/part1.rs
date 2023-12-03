fn main() {
    let input = include_str!("./input1.txt");
    let output = process(input);
    dbg!(output);
}

fn parse_string(input: &str) -> Vec<Vec<i32>> {
    input
        .lines()
        .map(|line| {
            let mut result: Vec<i32> = Vec::new();
            let mut current_number = String::new();

            for c in line.chars() {
                match c {
                    '0'..='9' => {
                        result.push(0);
                        current_number.push(c);
                    }
                    _ => {
                        if !current_number.is_empty() {
                            result.pop();
                            result.push(current_number.parse().unwrap());
                            current_number.clear();
                        }
                        result.push(if c.is_ascii_punctuation() && c != '.' {
                            -10
                        } else {
                            0
                        });
                    }
                }
            }
            if !current_number.is_empty() {
                result.pop();
                result.push(current_number.parse().unwrap());
            }
            result
        })
        .collect::<Vec<Vec<i32>>>()
}

fn is_part_number(matrix: &Vec<Vec<i32>>, row: usize, col: usize) -> bool {
    let is_valid_index = |r: usize, c: usize| r < matrix.len() && c < matrix[0].len();

    let neighbors = [
        (-1, 0),  // up
        (1, 0),   // down
        (0, -1),  // left
        (0, 1),   // right
        (-1, -1), // diagonal up-left
        (-1, 1),  // diagonal up-right
        (1, -1),  // diagonal down-left
        (1, 1),   // diagonal down-right
    ];

    neighbors.iter().any(|(dr, dc)| {
        let (r, c) = (row as isize + dr, col as isize + dc);
        is_valid_index(r as usize, c as usize) && matrix[r as usize][c as usize] < 0
    })
}

fn process(input: &str) -> u32 {
    let mut result = 0;
    let input_matrix = parse_string(input);
    let mut queue: Vec<(usize, usize)> = Vec::new();
    for (row_index, row) in input_matrix.iter().enumerate() {
        for (col_index, &value) in row.iter().enumerate() {
            if value > 0 {
                queue.push((row_index, col_index))
            }
        }
    }
    for (row, col) in queue {
        let is_valid = is_part_number(&input_matrix, row, col);
        if is_valid {
            println!("{}", input_matrix[row][col]);
            result += input_matrix[row][col] as u32;
        }
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() {
        let input = "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..";
        let result = process(input);
        assert_eq!(result, 4361);
    }
}
