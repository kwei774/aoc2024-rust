advent_of_code::solution!(7);

pub fn solve_equation(operands: &Vec<usize>, goal: usize, total: usize) -> bool {
    let operand = operands[0];
    let mut new_operands = operands.clone();
    new_operands.remove(0);
    // println!("running total: {}", total);
    if new_operands.len() == 0 {
        return (goal == total * operand) || (goal == total + operand)
    } else {
        return solve_equation(&new_operands, goal, total + operand) || solve_equation(&new_operands, goal, total * operand)
    }
}

pub fn solve_equation_append(operands: &Vec<usize>, goal: usize, total: usize) -> bool {
    let operand = operands[0];
    let mut new_operands = operands.clone();
    new_operands.remove(0);
    // println!("running total: {}", total);
    if new_operands.len() == 0 {
        return (goal == total * operand) || (goal == total + operand) || (goal == (total * (10 as usize).pow(operand.ilog10() + 1)) + operand)
    } else {
        return solve_equation_append(&new_operands, goal, total + operand) 
            || solve_equation_append(&new_operands, goal, total * operand) 
            || solve_equation_append(&new_operands, goal, (total * (10 as usize).pow(operand.ilog10() + 1)) + operand)
    }
}

pub fn part_one(input: &str) -> Option<usize> {
    let mut total_calibration = 0;
    for line in input.lines(){
        let stuff :Vec<&str> = line.split(": ").collect();
        let goal = stuff[0].parse::<usize>().unwrap();
        let mut operands :Vec<usize> = stuff[1].split(" ").map(|x| x.parse::<usize>().unwrap()).collect();
        // print!("goal: {} ", goal);
        let first = operands[0];
        operands.remove(0);
        // println!("{:?}", operands);
        if solve_equation(&operands, goal, first) {
            total_calibration += goal;
        }
    }

    Some(total_calibration)
}

pub fn part_two(input: &str) -> Option<usize> {
    let mut total_calibration = 0;
    for line in input.lines(){
        let stuff :Vec<&str> = line.split(": ").collect();
        let goal = stuff[0].parse::<usize>().unwrap();
        let mut operands :Vec<usize> = stuff[1].split(" ").map(|x| x.parse::<usize>().unwrap()).collect();
        // print!("goal: {} ", goal);
        let first = operands[0];
        operands.remove(0);
        // println!("{:?}", operands);
        if solve_equation_append(&operands, goal, first) {
            total_calibration += goal;
        }
    }

    Some(total_calibration)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3749));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(11387));
    }
}
