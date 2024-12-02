advent_of_code::solution!(2);

pub fn part_one(input: &str) -> Option<i32> {
    let mut sum_safe = 0;
    let reports: Vec<&str> = input.split('\n').collect();
    for report in reports{
        let levels_num : Result<Vec<i32>, _> = report.split(" ").map(|x| x.parse()).collect();
        // println!("{:?}", levels_num);
        let mut increase = false;
        let first_level = levels_num.as_ref().unwrap()[0];
        let second_level = levels_num.as_ref().unwrap()[1];
        // println!("first: {}, second: {}", first_level, second_level);
        if second_level - first_level > 0 {
            increase = true;
            // println!("increasing");
        } else if second_level - first_level < 0 {
            increase = false;
            // println!("decreasing");
        } else {
            continue;
        }
        let mut previous_level = first_level;
        let mut counter = 1;
        for level in &levels_num.as_ref().unwrap()[1..]{
            if increase && level > &previous_level && level - previous_level >= 1 && level - previous_level <= 3{
                previous_level = *level;
                counter += 1;
                if counter == levels_num.as_ref().unwrap().len(){
                    sum_safe += 1;
                }
            } else if !increase && level < &previous_level && level - previous_level <= -1 && level - previous_level >= -3{
                previous_level = *level;
                counter += 1;
                if counter == levels_num.as_ref().unwrap().len(){
                    sum_safe += 1;
                }
            } else {
                continue;
            }
        }
    }
    Some(sum_safe)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut sum_safe = 0;
    let reports: Vec<&str> = input.split('\n').collect();
    for report in reports{
        let levels_num : Result<Vec<i32>, _> = report.split(" ").map(|x| x.parse()).collect();
        // println!("{:?}", levels_num);
        let mut increase = false;
        let first_level = levels_num.as_ref().unwrap()[0];
        let second_level = levels_num.as_ref().unwrap()[1];
        let third_level = levels_num.as_ref().unwrap()[2];
        let mut used_dampener = false;
        // println!("first: {}, second: {}", first_level, second_level);
        if second_level - first_level > 0 {
            increase = true;
        } else if second_level - first_level < 0 {
            increase = false;
        } else if third_level - first_level > 0 {
            increase = true;
            used_dampener = true;
        } else if third_level - first_level < 0 {
            increase = false;
            used_dampener = true;
        } else {
            continue;
        }
        let mut previous_level = first_level;
        let mut counter = 1;
        if used_dampener{
            counter += 1;
            for level in &levels_num.as_ref().unwrap()[2..]{
                if increase && level > &previous_level && level - previous_level >= 1 && level - previous_level <= 3{
                    previous_level = *level;
                    counter += 1;
                    if counter == levels_num.as_ref().unwrap().len(){
                        sum_safe += 1;
                    }
                } else if !increase && level < &previous_level && level - previous_level <= -1 && level - previous_level >= -3{
                    previous_level = *level;
                    counter += 1;
                    if counter == levels_num.as_ref().unwrap().len(){
                        sum_safe += 1;
                    }
                }
                else {
                    println!("{:?}", levels_num.as_ref().unwrap());
                    break;
                }
            }

        }
        else {
            for level in &levels_num.as_ref().unwrap()[1..]{
                if increase && level > &previous_level && level - previous_level >= 1 && level - previous_level <= 3{
                    previous_level = *level;
                    counter += 1;
                    if counter == levels_num.as_ref().unwrap().len(){
                        sum_safe += 1;
                    }
                } else if !increase && level < &previous_level && level - previous_level <= -1 && level - previous_level >= -3{
                    previous_level = *level;
                    counter += 1;
                    if counter == levels_num.as_ref().unwrap().len(){
                        sum_safe += 1;
                    }
                } else if used_dampener{
                    println!("{:?}", levels_num.as_ref().unwrap());
                    break;
                } else {
                    used_dampener = true;
                    counter += 1;
                    if counter == levels_num.as_ref().unwrap().len(){
                        sum_safe += 1;
                    }
                }
            }
        }
    }
    println!("{}", sum_safe);
    Some(sum_safe)
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
