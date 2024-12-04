use regex::Regex;

advent_of_code::solution!(3);

pub fn part_one(input: &str) -> Option<u32> {
    let mut sum_multiplication = 0;
    let re=Regex::new(r"mul\((\d*)\,(\d*)\)").unwrap();
    // println!("{}", row);
    for mat in re.captures_iter(input){
        // println!("{}, {}, {}", &mat[0],&mat[1], &mat[2]);
        sum_multiplication += &mat[1].parse::<u32>().unwrap() * &mat[2].parse::<u32>().unwrap();
    }
    Some(sum_multiplication)
}


pub fn part_two(input: &str) -> Option<u32> {
    let mut sum_multiplication = 0;
    let mul_pat=Regex::new(r"mul\((\d*)\,(\d*)\)").unwrap();
    let remove_pat=Regex::new(r"don't\(\)[\S\s]*?do\(\)").unwrap();
    let input_filtered = remove_pat.replace_all(input, ".");
    // println!("{}", new_row);
    for mat in mul_pat.captures_iter(&input_filtered){
        // println!("{}, {}, {}", &mat[0],&mat[1], &mat[2]);
        // println!("{}, {}", &mat[1], &mat[2]);
        sum_multiplication += &mat[1].parse::<u32>().unwrap() * &mat[2].parse::<u32>().unwrap();
    }

    Some(sum_multiplication)
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
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(48));
    }
}
