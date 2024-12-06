use std::collections::{HashMap, HashSet, VecDeque};
advent_of_code::solution!(5);

fn is_valid_order(rules: &HashMap<u32, HashSet<u32>>, update: &[u32]) -> bool {
    for (i, &num) in update.iter().enumerate() {
        if let Some(must_follow) = rules.get(&num) {
            for &req in must_follow {
                if update[i+1..].contains(&req) {
                    return false;
                }
            }
        }
    }
    true
}

fn topological_sort(rules: &HashMap<u32, HashSet<u32>>, numbers: &[u32]) -> Vec<u32> {
    let mut result = Vec::new();
    let mut in_degree: HashMap<u32, usize> = HashMap::new();
    let mut graph: HashMap<u32, Vec<u32>> = HashMap::new();
    let mut queue = VecDeque::new();

    // Build graph for this specific update
    for &num in numbers {
        in_degree.insert(num, 0);
        graph.entry(num).or_default();
    }

    // Calculate in-degrees
    for &num in numbers {
        if let Some(deps) = rules.get(&num) {
            for &dep in deps {
                if numbers.contains(&dep) {
                    graph.entry(dep).or_default().push(num);
                    *in_degree.entry(num).or_default() += 1;
                }
            }
        }
    }

    // Add nodes with 0 in-degree to queue
    for &num in numbers {
        if in_degree[&num] == 0 {
            queue.push_back(num);
        }
    }

    // Process queue
    while let Some(num) = queue.pop_front() {
        result.push(num);
        if let Some(neighbors) = graph.get(&num) {
            for &next in neighbors {
                *in_degree.get_mut(&next).unwrap() -= 1;
                if in_degree[&next] == 0 {
                    queue.push_back(next);
                }
            }
        }
    }

    result
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut sections = input.split("\n\n");
    let rules_str = sections.next()?;
    let updates_str = sections.next()?;

    // Build rules graph
    let mut rules: HashMap<u32, HashSet<u32>> = HashMap::new();
    for line in rules_str.lines() {
        let mut parts = line.split('|');
        let before: u32 = parts.next()?.parse().ok()?;
        let after: u32 = parts.next()?.parse().ok()?;
        rules.entry(after).or_default().insert(before);
    }

    // Process updates
    let mut sum = 0;
    for update in updates_str.lines() {
        let numbers: Vec<u32> = update.split(',')
            .filter_map(|n| n.parse().ok())
            .collect();

        if is_valid_order(&rules, &numbers) {
            let mid = numbers[numbers.len() / 2];
            sum += mid;
        }
    }

    Some(sum)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut sections = input.split("\n\n");
    let rules_str = sections.next()?;
    let updates_str = sections.next()?;

    // Build rules graph (reuse from part one)
    let mut rules: HashMap<u32, HashSet<u32>> = HashMap::new();
    for line in rules_str.lines() {
        let mut parts = line.split('|');
        let before: u32 = parts.next()?.parse().ok()?;
        let after: u32 = parts.next()?.parse().ok()?;
        rules.entry(after).or_default().insert(before);
    }

    let mut sum = 0;
    for update in updates_str.lines() {
        let numbers: Vec<u32> = update.split(',')
            .filter_map(|n| n.parse().ok())
            .collect();

        if !is_valid_order(&rules, &numbers) {
            let sorted = topological_sort(&rules, &numbers);
            sum += sorted[sorted.len() / 2];
        }
    }

    Some(sum)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(143));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(123));
    }
}
