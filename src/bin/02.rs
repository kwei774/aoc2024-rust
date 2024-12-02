advent_of_code::solution!(2);

pub fn safe_level(input: &Vec<i32>) -> bool {
    let mut levels_ascending = input.clone();
    levels_ascending.sort();
    let mut levels_descending = levels_ascending.clone();
    levels_descending.reverse();
    if &levels_ascending != input && &levels_descending != input {
        return false;
    }
    let mut previous_level = input[0];
    for level in &input[1..]{
        if (level - previous_level).abs() >= 1 && (level - previous_level).abs() <= 3 {
            previous_level = *level;
        } else {
            return false;
        }
    }
    return true;
}

pub fn part_one(input: &str) -> Option<i32> {
    let mut sum_safe = 0;
    let reports: Vec<&str> = input.split('\n').collect();
    for report in reports{
        let levels_result : Result<Vec<i32>, _> = report.split(" ").map(|x| x.parse::<i32>()).collect();
        
        if safe_level(levels_result.as_ref().unwrap()) {
            // println!("{:?}", levels_result.as_ref().unwrap());
            sum_safe += 1;
        }
    }
    Some(sum_safe)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut sum_safe = 0;
    let reports: Vec<&str> = input.split('\n').collect();
    for report in reports{
        let levels_result : Result<Vec<i32>, _> = report.split(" ").map(|x| x.parse::<i32>()).collect();
        
        if safe_level(levels_result.as_ref().unwrap()) {
            // println!("{:?}", levels_result.as_ref().unwrap());
            sum_safe += 1;
        } else {
            for i in 0..levels_result.as_ref().unwrap().len(){
                let mut brute_force_vector = levels_result.as_ref().unwrap().clone();
                brute_force_vector.remove(i);
                if safe_level(&brute_force_vector){
                    sum_safe += 1;
                    break;
                }
            }
        }
    }
    // println!("{}", sum_safe);
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
