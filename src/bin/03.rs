advent_of_code::solution!(3);

use regex::Regex;

pub fn calculate_total_from_regex(regex: Regex, input: &str) -> u32 {
    let mut total = 0;
    for caps in regex.captures_iter(input) {
        let a = caps[1].parse::<u32>().unwrap();
        let b = caps[2].parse::<u32>().unwrap();
        total += a * b;
    }
    total
}

pub fn part_one(input: &str) -> Option<u32> {
    let re = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();

    Some(calculate_total_from_regex(re.clone(), input))
}

pub fn part_two(input: &str) -> Option<u32> {
    let re = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();
    let control_re = Regex::new(r"(do|don't)\(\)").unwrap();

    let mut total = 0;
    let mut last_pos = 0;
    let mut previous_control = "do()"; // Start enabled

    for control_match in control_re.find_iter(input){
        let control = control_match.as_str();
        let pos = control_match.start();
        if previous_control == "do()" {
            total += calculate_total_from_regex(re.clone(), &input[last_pos..pos]);
        }

        previous_control = control;
        last_pos = pos;
    }

    // Finish off the rest of the input
    if last_pos != input.len(){
        total += calculate_total_from_regex(re.clone(), &input[last_pos..]);
    }

    Some(total)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(161));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file_part("examples", DAY, 2));
        assert_eq!(result, Some(48));
    }
}
