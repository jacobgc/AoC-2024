advent_of_code::solution!(4);

pub fn part_one(input: &str) -> Option<u32> {
    let grid: Vec<Vec<char>> = input
        .lines()
        .map(|line| line.chars().collect())
        .collect();

    let rows = grid.len();
    let cols = grid[0].len();
    let target = vec!['X', 'M', 'A', 'S'];
    let mut count = 0;

    // Check all possible starting positions
    for i in 0..rows {
        for j in 0..cols {
            // Check all 8 directions
            let directions = [
                (0, 1),   // right
                (0, -1),  // left
                (1, 0),   // down
                (-1, 0),  // up
                (1, 1),   // down-right
                (-1, -1), // up-left
                (1, -1),  // down-left
                (-1, 1),  // up-right
            ];

            for (direction_row, direction_column) in directions {
                let mut found = true;
                for character_pos in 0..4 {
                    let new_row = i as i32 + direction_row * character_pos;
                    let new_column = j as i32 + direction_column * character_pos;

                    // Check if we can fit the word in this direction
                    if new_row < 0 || new_row >= rows as i32 ||
                       new_column < 0 || new_column >= cols as i32 {
                        found = false;
                        break;
                    }

                    // Check if the character is what we're expecting
                    let current_char = grid[new_row as usize][new_column as usize];
                    if current_char != target[character_pos as usize] {
                        found = false;
                        break;
                    }
                }
                if found {
                    count += 1;
                }
            }
        }
    }

    Some(count)
}

pub fn part_two(input: &str) -> Option<u32> {
    let grid: Vec<Vec<char>> = input
        .lines()
        .map(|line| line.chars().collect())
        .collect();

    let rows = grid.len();
    let cols = grid[0].len();
    let mut count = 0;

    // Find each A in the grid
    for row in 0..rows {
        for col in 0..cols {
            if grid[row][col] == 'A' {
                let mut tmp_count = 0;
                // Check if we can access all required surrounding positions
                if row == 0 || row >= rows - 1 || col == 0 || col >= cols - 1 {
                    continue;
                }

                // Check if two of these if statements pass.
                if grid[row-1][col-1] == 'M' && grid[row+1][col+1] == 'S' {
                    tmp_count+=1;
                }
                if grid[row-1][col-1] == 'S' && grid[row+1][col+1] == 'M' {
                    tmp_count+=1;
                }
                if grid[row+1][col-1] == 'M' && grid[row-1][col+1] == 'S' {
                    tmp_count+=1;
                }
                if grid[row+1][col-1] == 'S' && grid[row-1][col+1] == 'M' {
                    tmp_count+=1;
                }

                if tmp_count == 2 {
                    count += 1;
                }

            }
        }
    }


    Some(count)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file_part("examples", DAY, 2));
        assert_eq!(result, Some(9));
    }
}
