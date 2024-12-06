advent_of_code::solution!(5);

use std::collections::HashMap;

pub fn find_middle(input: &Vec<&str>) -> u32 {
    input.get(input.len() / 2).unwrap().parse::<u32>().unwrap()
}

pub fn part_one(input: &str) -> Option<u32> {
    let parts: Vec<&str> = input.split("\n\n").collect();
    // println!("{:?}", parts.get(0).unwrap());
    let all_rules: Vec<&str> = parts.get(0).unwrap().split("\n").collect();
    let mut rule_pairs: Vec<(&str, &str)> = Vec::new();
    for rule in all_rules {
        let num_str: Vec<&str> = rule.split("|").collect();
        rule_pairs.push((num_str.get(0).unwrap(), num_str.get(1).unwrap()));
    }
    // for debug in rule_pairs {
    //     println!("{}, {}", debug.0, debug.1);
    // }
    let all_updates: Vec<&str> = parts.get(1).unwrap().split("\n").collect();
    let mut sum_mid = 0;
    for update_list in all_updates {
        // println!("{:?}", update_list);
        let mut passes = true;
        for rule in &rule_pairs {
            let match_first: Vec<_> = update_list.match_indices(rule.0).collect();
            let match_second: Vec<_> = update_list.match_indices(rule.1).collect();
            if match_first.len() > 0 && match_second.len() > 0 && match_first.get(0).unwrap().0 > match_second.get(0).unwrap().0 {
                passes = false;
                break;
            }
        }
        if passes{
            sum_mid += find_middle(&update_list.split(",").collect())
        }
    }
    Some(sum_mid)
}

pub fn part_two(input: &str) -> Option<usize> {
    let parts: Vec<&str> = input.split("\n\n").collect();
    // println!("{:?}", parts.get(0).unwrap());
    let all_rules: Vec<&str> = parts.get(0).unwrap().split("\n").collect();
    let mut rule_map: HashMap<usize, Vec<usize>> = HashMap::new();
    for rule in all_rules {
        let num_str: Vec<&str> = rule.split("|").collect();
        let first_num = &num_str.get(0).unwrap().parse::<usize>().unwrap();
        let second_num = &num_str.get(1).unwrap().parse::<usize>().unwrap();
        if rule_map.contains_key(second_num) {
            rule_map.entry(*second_num).and_modify(|v| v.push(*first_num));
        } else {
            rule_map.insert(*second_num, Vec::from([*first_num]));
        }
    }
    // for (key, val) in &rule_map {
    //     println!("{}, {:?}", key, val);
    // }
    let all_updates: Vec<&str> = parts.get(1).unwrap().split("\n").collect();
    let mut sum_mid = 0;
    for update_list in all_updates {
        let mut update_list_vec : Vec<_> = update_list.split(",").flat_map(|x| x.parse()).collect();
        // if update_list_vec
        // println!("{:?}", update_list_vec);
        if !update_list_vec.is_sorted_by(|a, b| rule_map[b].contains(a)) {
            // println!("old: {:?}", update_list_vec);
            update_list_vec.sort_by(|a, b| rule_map[b].contains(a).cmp(&true));
            // println!("new: {:?}", update_list_vec);
            sum_mid += update_list_vec[update_list_vec.len() / 2];
        }
    }
    Some(sum_mid)
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_find_middle() {
        let result = find_middle(&Vec::from(["10", "1", "11"]));
        assert_eq!(result, 1);
    }

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
