advent_of_code::solution!(4);

pub fn check_xmas(search: &Vec<Vec<char>>, x : usize, y : usize) -> u32 {
    let max = search.len() - 1;
    let mut xmas_count = 0;
    if x + 3 <= max { //right
        if *search.get(y).unwrap().get(x + 1).unwrap() == 'M' 
            && *search.get(y).unwrap().get(x + 2).unwrap() == 'A' 
            && *search.get(y).unwrap().get(x + 3).unwrap() == 'S' {
            xmas_count += 1;
        }
    }
    if x + 3 <= max && y + 3 <= max{ //right down
        if *search.get(y + 1).unwrap().get(x + 1).unwrap() == 'M' 
            && *search.get(y + 2).unwrap().get(x + 2).unwrap() == 'A' 
            && *search.get(y + 3).unwrap().get(x + 3).unwrap() == 'S' {
            xmas_count += 1;
        }
    }
    if y + 3 <= max{ //down
        if *search.get(y + 1).unwrap().get(x).unwrap() == 'M' 
            && *search.get(y + 2).unwrap().get(x).unwrap() == 'A' 
            && *search.get(y + 3).unwrap().get(x).unwrap() == 'S' {
            xmas_count += 1;
        }
    }
    if y + 3 <= max && x >= 3 { //down left
        if *search.get(y + 1).unwrap().get(x - 1).unwrap() == 'M' 
            && *search.get(y + 2).unwrap().get(x - 2).unwrap() == 'A' 
            && *search.get(y + 3).unwrap().get(x - 3).unwrap() == 'S' {
            xmas_count += 1;
        }
    }
    if x >= 3 { //left
        if *search.get(y).unwrap().get(x - 1).unwrap() == 'M' 
            && *search.get(y).unwrap().get(x - 2).unwrap() == 'A' 
            && *search.get(y).unwrap().get(x - 3).unwrap() == 'S' {
            xmas_count += 1;
        }
    }
    if y >= 3 && x >= 3 { //left up
        if *search.get(y - 1).unwrap().get(x - 1).unwrap() == 'M' 
            && *search.get(y - 2).unwrap().get(x - 2).unwrap() == 'A' 
            && *search.get(y - 3).unwrap().get(x - 3).unwrap() == 'S' {
            xmas_count += 1;
        }
    }
    if y >= 3 { //up
        if *search.get(y - 1).unwrap().get(x).unwrap() == 'M' 
            && *search.get(y - 2).unwrap().get(x).unwrap() == 'A' 
            && *search.get(y - 3).unwrap().get(x).unwrap() == 'S' {
            xmas_count += 1;
        }
    }
    if y >= 3 && x + 3 <= max { //up right
        if *search.get(y - 1).unwrap().get(x + 1).unwrap() == 'M' 
            && *search.get(y - 2).unwrap().get(x + 2).unwrap() == 'A' 
            && *search.get(y - 3).unwrap().get(x + 3).unwrap() == 'S' {
            xmas_count += 1;
        }
    }
    xmas_count
}


pub fn part_one(input: &str) -> Option<u32> {
    let rows: Vec<&str> = input.split('\n').collect();
    let mut word_search : Vec<Vec<char>> = Vec::new();
    let mut xmas_count = 0;
    for row in rows{
        let row_contents: Vec<char> = row.chars().collect();
        word_search.push(row_contents);
    }
    for y in 0..word_search.len(){
        for x in 0..word_search.get(y).unwrap().len(){
            if *word_search.get(y).unwrap().get(x).unwrap() == 'X'{
                xmas_count += check_xmas(&word_search, x, y);
            }
            // print!("{}", word_search.get(y).unwrap().get(x).unwrap());
        }
        // print!("\n");
    }
    Some(xmas_count)
}
pub fn grid_mas(search: &Vec<Vec<char>>, x : usize, y : usize) -> u32 {
    let mut xmas_count = 0;
    if *search.get(y - 1).unwrap().get(x - 1).unwrap() == 'M'
        && *search.get(y - 1).unwrap().get(x + 1).unwrap() == 'M'
        && *search.get(y + 1).unwrap().get(x - 1).unwrap() == 'S'
        && *search.get(y + 1).unwrap().get(x + 1).unwrap() == 'S' {
            xmas_count += 1
    }
    if *search.get(y - 1).unwrap().get(x - 1).unwrap() == 'S'
        && *search.get(y - 1).unwrap().get(x + 1).unwrap() == 'S'
        && *search.get(y + 1).unwrap().get(x - 1).unwrap() == 'M'
        && *search.get(y + 1).unwrap().get(x + 1).unwrap() == 'M' {
            xmas_count += 1
    }
    if *search.get(y - 1).unwrap().get(x - 1).unwrap() == 'S'
        && *search.get(y - 1).unwrap().get(x + 1).unwrap() == 'M'
        && *search.get(y + 1).unwrap().get(x - 1).unwrap() == 'S'
        && *search.get(y + 1).unwrap().get(x + 1).unwrap() == 'M' {
            xmas_count += 1
    }
    if *search.get(y - 1).unwrap().get(x - 1).unwrap() == 'M'
        && *search.get(y - 1).unwrap().get(x + 1).unwrap() == 'S'
        && *search.get(y + 1).unwrap().get(x - 1).unwrap() == 'M'
        && *search.get(y + 1).unwrap().get(x + 1).unwrap() == 'S' {
            xmas_count += 1
    }
    xmas_count
}

pub fn part_two(input: &str) -> Option<u32> {
    let rows: Vec<&str> = input.split('\n').collect();
    let mut word_search : Vec<Vec<char>> = Vec::new();
    let mut xmas_count = 0;
    for row in rows{
        let row_contents: Vec<char> = row.chars().collect();
        word_search.push(row_contents);
    }
    for y in 1..word_search.len() - 1{
        for x in 1..word_search.get(y).unwrap().len() - 1{
            if *word_search.get(y).unwrap().get(x).unwrap() == 'A'{
                xmas_count += grid_mas(&word_search, x, y);
            }
            // print!("{}", word_search.get(y).unwrap().get(x).unwrap());
        }
        // print!("\n");
    }
    Some(xmas_count)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(18));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(9));
    }
}
