advent_of_code::solution!(1);

fn parse_input(input: &str) -> (Vec<i32>, Vec<i32>) {
    let mut left_side = Vec::new();
    let mut right_side = Vec::new();

    for line in input.lines().filter(|l| !l.is_empty()) {
        let parts: Vec<&str> = line.split("   ").map(|s| s.trim()).collect();
        if parts.len() == 2 {
            left_side.push(parts[0].parse::<i32>().unwrap());
            right_side.push(parts[1].parse::<i32>().unwrap());
        }
    }

    left_side.sort();
    right_side.sort();
    (left_side, right_side)
}

pub fn part_one(input: &str) -> Option<u32> {
    let (left_side, right_side) = parse_input(input);
    Some(
        left_side
            .iter()
            .zip(right_side.iter())
            .map(|(l, r)| i32::abs(l - r) as u32)
            .sum(),
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    let (left_side, right_side) = parse_input(input);
    Some(
        left_side
            .iter()
            .map(|&left| {
                let count = right_side.iter().filter(|&&right| left == right).count();
                (left * count as i32) as u32
            })
            .sum(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(11));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(31));
    }
}
