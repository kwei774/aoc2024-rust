advent_of_code::solution!(1);

use std::num;

pub fn part_one(input: &str) -> Option<i32> {
    let mut sum = 0;
    let lines: Vec<&str> = input.split('\n').collect();
    let mut left_list: Vec<i32> = Vec::new();
    let mut right_list: Vec<i32> = Vec::new();
    for line in lines{
        let split: Vec<&str> = line.split("   ").collect();
        // println!("left: {}, right: {}", split[0], split[1]);

        left_list.push(split[0].parse::<i32>().unwrap());
        right_list.push(split[1].parse::<i32>().unwrap());
    }
    left_list.sort();
    right_list.sort();
    let mut counter = 0;
    let mut sum = 0;
    for item in left_list{
        // println!("left: {}, right: {}", item, right_list[counter]);
        sum += (item - right_list[counter]).abs();
        counter += 1;
    }
    Some(sum)
}

pub fn part_two(input: &str) -> Option<i32> {
    let mut sum = 0;
    let lines: Vec<&str> = input.split('\n').collect();
    let mut left_list: Vec<i32> = Vec::new();
    let mut right_list: Vec<i32> = Vec::new();
    for line in lines{
        let split: Vec<&str> = line.split("   ").collect();
        // println!("left: {}, right: {}", split[0], split[1]);

        left_list.push(split[0].parse::<i32>().unwrap());
        right_list.push(split[1].parse::<i32>().unwrap());
    }
    left_list.sort();
    right_list.sort();
    let mut sum = 0;
    for left_item in &left_list{
        let mut similarity = 0;
        for right_item in &right_list{
            if (left_item == right_item){
                similarity += 1;
            }
        }
        sum += left_item * similarity;
    }
    Some(sum)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
