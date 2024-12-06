use std::collections::HashSet;

advent_of_code::solution!(6);

#[derive(Debug, Clone)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug)]
struct Guard {
    row: usize,
    col: usize,
    direction: Direction,
}

impl Guard {
    fn from_char(c: char, row: usize, col: usize) -> Option<Self> {
        let direction = match c {
            '^' => Some(Direction::Up),
            'v' => Some(Direction::Down),
            '<' => Some(Direction::Left),
            '>' => Some(Direction::Right),
            _ => None,
        };

        direction.map(|d| Guard {
            row,
            col,
            direction: d,
        })
    }

    fn rotate_right(&mut self) {
        self.direction = match self.direction {
            Direction::Up => Direction::Right,
            Direction::Right => Direction::Down,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
        };
    }

    fn next_position(&self) -> (usize, usize) {
        match self.direction {
            Direction::Up => (self.row.wrapping_sub(1), self.col),
            Direction::Down => (self.row + 1, self.col),
            Direction::Left => (self.row, self.col.wrapping_sub(1)),
            Direction::Right => (self.row, self.col + 1),
        }
    }

    fn move_forward(&mut self, next: (usize, usize)) {
        self.row = next.0;
        self.col = next.1;
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let grid: Vec<Vec<char>> = input
        .lines()
        .map(|line| line.chars().collect())
        .collect();
    let grid_height = grid.len();
    let width = grid[0].len();

    let mut guard = grid.iter().enumerate()
        .find_map(|(row, line)| {
            line.iter().enumerate()
                .find_map(|(col, &c)| Guard::from_char(c, row, col))
        })?;

    let mut visited = HashSet::new();
    visited.insert((guard.row, guard.col));

    loop {
        let next = guard.next_position();

        if next.0 >= grid_height || next.1 >= width {
            break;
        }

        if grid[next.0][next.1] == '#' {
            guard.rotate_right();
        } else {
            guard.move_forward(next);
            visited.insert((guard.row, guard.col));
        }
    }

    Some(visited.len() as u32)
}

fn simulate_with_obstacle(
    original_grid: &Vec<Vec<char>>,
    obstacle_pos: (usize, usize),
    start_guard: &Guard,
) -> Option<bool> {
    let height = original_grid.len();
    let width = original_grid[0].len();

    // Create a modified grid with the new obstacle
    let mut grid = original_grid.clone();
    if grid[obstacle_pos.0][obstacle_pos.1] != '.' {
        return None;
    }
    grid[obstacle_pos.0][obstacle_pos.1] = '#';

    let mut guard = Guard {
        row: start_guard.row,
        col: start_guard.col,
        direction: start_guard.direction.clone(),
    };

    let mut visited = Vec::new();
    let mut positions = HashSet::new();

    loop {
        let state = (guard.row, guard.col,
            std::mem::discriminant(&guard.direction));

        if positions.contains(&state) {
            return Some(true); // Found a loop
        }

        positions.insert(state);
        visited.push((guard.row, guard.col));

        let next = guard.next_position();
        if next.0 >= height || next.1 >= width {
            return Some(false); // Guard escaped
        }

        if grid[next.0][next.1] == '#' {
            guard.rotate_right();
        } else {
            guard.move_forward(next);
        }
    }
}

pub fn part_two(input: &str) -> Option<u32> {
    let grid: Vec<Vec<char>> = input
        .lines()
        .map(|line| line.chars().collect())
        .collect();

    let start_guard = grid.iter().enumerate()
        .find_map(|(row, line)| {
            line.iter().enumerate()
                .find_map(|(col, &c)| Guard::from_char(c, row, col))
        })?;

    let height = grid.len();
    let width = grid[0].len();
    let mut loop_count = 0;

    for row in 0..height {
        for col in 0..width {
            if (row, col) != (start_guard.row, start_guard.col) {
                if let Some(true) = simulate_with_obstacle(&grid, (row, col), &start_guard) {
                    loop_count += 1;
                }
            }
        }
    }

    Some(loop_count)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(41));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(6));
    }
}
