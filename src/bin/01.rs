advent_of_code::solution!(1);

pub fn part_one(input: &str) -> Option<u32> {
    let split_input_by_line = input.split("\n");
    let mut left_side: Vec<i32> = Vec::new();
    let mut right_side: Vec<i32> = Vec::new();

    for ele in split_input_by_line {
        if ele.is_empty() {
            continue;
        }

        let mut split_element = ele.split("   ");
        let left_element = split_element.next().unwrap();
        let right_element = split_element.next().unwrap();

        left_side.push(left_element.parse::<i32>().unwrap());
        right_side.push(right_element.parse::<i32>().unwrap());
    }

    left_side.sort();
    right_side.sort();

    let mut sum = 0;
    for i in 0..left_side.len() {
        let diff = i32::abs(left_side.get(i).unwrap() - right_side.get(i).unwrap());
        sum += diff;
    }

    Some(sum as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    let split_input_by_line = input.split("\n");
    let mut left_side: Vec<i32> = Vec::new();
    let mut right_side: Vec<i32> = Vec::new();

    for ele in split_input_by_line {
        if ele.is_empty() {
            continue;
        }

        let mut split_element = ele.split("   ");
        let left_element = split_element.next().unwrap();
        let right_element = split_element.next().unwrap();

        left_side.push(left_element.parse::<i32>().unwrap());
        right_side.push(right_element.parse::<i32>().unwrap());
    }

    left_side.sort();
    right_side.sort();

    let mut total = 0;
    for i in 0..left_side.len() {
        let left = left_side.get(i).unwrap();
        let mut found = 0;

        for j in 0..right_side.len() {
            let right = right_side.get(j).unwrap();
            if left == right {
                found += 1;
            }
        }
        total += left * found;
    }
    Some(total as u32)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        println!("{:?}", result);
        assert_eq!(result, Some(11));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        println!("{:?}", result);
        assert_eq!(result, Some(31));
    }
}
