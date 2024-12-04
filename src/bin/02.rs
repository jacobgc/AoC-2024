advent_of_code::solution!(2);

fn parse_input(input: &str) -> Vec<Vec<i32>> {
    let mut input_vec: Vec<Vec<i32>> = Vec::new();
    input.lines().for_each(|line| {
        input_vec.push(line.split_whitespace().map(|x| x.parse::<i32>().unwrap()).collect());
    });
    input_vec
}

pub fn part_one(input: &str) -> Option<u32> {
    let input_vec = parse_input(input);

    let mut total = 0;

    // Ensure each input is safe. Elements must all be accending or decending and must change by at minimum 1 and maximum 3
    for ele in input_vec {
        let is_ascending = ele.windows(2).all(|w| w[0] < w[1]);
        let is_descending = ele.windows(2).all(|w| w[0] > w[1]);

        if !is_ascending && !is_descending {
            continue;
        }

        let is_each_element_safe = ele.windows(2).all(|w| {
            let diff = i32::abs(w[1] - w[0]);
            (1..=3).contains(&diff)
        });
        if is_each_element_safe{
            total+=1;
        }
    }

    Some(total)
}

pub fn part_two(input: &str) -> Option<u32> {
    let input_vec = parse_input(input);

    let mut total = 0;

    for ele in input_vec {
        let is_ascending = ele.windows(2).all(|w| w[0] < w[1]);
        let is_descending = ele.windows(2).all(|w| w[0] > w[1]);

        if is_ascending || is_descending {
            let is_each_element_safe = ele.windows(2).all(|w| {
                let diff = i32::abs(w[1] - w[0]);
                (1..=3).contains(&diff)
            });
            if is_each_element_safe {
                total += 1;
                continue;
            }
        }

        for i in 0..ele.len() {
            let mut temp = ele.clone();
            temp.remove(i);

            let is_ascending = temp.windows(2).all(|w| w[0] < w[1]);
            let is_descending = temp.windows(2).all(|w| w[0] > w[1]);

            if is_ascending || is_descending {
                let is_each_element_safe = temp.windows(2).all(|w| {
                    let diff = i32::abs(w[1] - w[0]);
                    (1..=3).contains(&diff)
                });
                if is_each_element_safe {
                    total += 1;
                    break;
                }
            }
        }
    }

    Some(total)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4));
    }
}
