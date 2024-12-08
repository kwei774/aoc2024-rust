use std::collections::HashMap;

advent_of_code::solution!(8);

pub fn part_one(input: &str) -> Option<u32> {
    
    let rows : Vec<&str> = input.lines().collect();
    let map_limit = rows.len();
    let mut antinode_map : Vec<Vec<char>> = vec![vec!['.'; map_limit]; map_limit];
    let mut antenna_map : HashMap<char, Vec<(usize, usize)>> = HashMap::new();
    
    for row in 0..map_limit {
        for column in 0..map_limit {
            let point = rows[row].chars().nth(column).unwrap();
            antinode_map[row][column] = point;
            if point.is_alphanumeric() {
                if antenna_map.get(&point).is_none(){
                    let mut antenna_list :Vec<(usize, usize)> = Vec::new();
                    antenna_list.push((row, column));
                    antenna_map.insert(point, antenna_list);
                } else {
                    let antenna_list: &mut Vec<(usize, usize)> = antenna_map.get_mut(&point).unwrap();
                    antenna_list.push((row, column));
                }
            }
        }
    }
    
    for (key, val) in antenna_map.iter() {
        // println!("key: {} value: {:?}", key, val);
        for point1 in 0..val.len() {
            for point2 in point1..val.len() {
                if point1 != point2 {
                    let distance_row = val[point2].0 as isize - val[point1].0 as isize;
                    let distance_column = val[point2].1 as isize - val[point1].1 as isize;
                    let antinode_1_row = val[point1].0.checked_add_signed(-1 * distance_row);
                    let antinode_1_column = val[point1].1.checked_add_signed(-1 * distance_column);
                    let antinode_2_row = val[point2].0.checked_add_signed(distance_row);
                    let antinode_2_column = val[point2].1.checked_add_signed(distance_column);
                    // println!("point1: {:?}; point2: {:?}; distance_row: {}; distance_column: {}", val[point1], val[point2], distance_row, distance_column);
                    if antinode_1_row.is_some() 
                        && antinode_1_row.unwrap() < map_limit 
                        && antinode_1_column.is_some() 
                        && antinode_1_column.unwrap() < map_limit {
                        antinode_map[antinode_1_row.unwrap()][antinode_1_column.unwrap()] = '#';
                        // println!("antinode 1: {}, {}", antinode_1_row.unwrap(), antinode_1_column.unwrap());
                    }
                    if antinode_2_row.is_some() 
                        && antinode_2_row.unwrap() < map_limit 
                        && antinode_2_column.is_some() 
                        && antinode_2_column.unwrap() < map_limit {
                        antinode_map[antinode_2_row.unwrap()][antinode_2_column.unwrap()] = '#';
                        // println!("antinode 2: {}, {}", antinode_2_row.unwrap(), antinode_2_column.unwrap());
                    }
                    // for row in 0..antinode_map.len() {
                    //     for column in 0..antinode_map.len() {
                    //         print!("{}", antinode_map[row][column]);
                    //     }
                    //     println!();
                    // }
                }
            }
        }
    }
    
    let mut sum_antinode = 0;
    for row in 0..antinode_map.len() {
        for column in 0..antinode_map.len() {
            // print!("{}", antinode_map[row][column]);
            if antinode_map[row][column] == '#' {
                sum_antinode += 1;
            }
        }
        // println!();
    }
    Some(sum_antinode)
}

pub fn part_two(input: &str) -> Option<u32> {
    
    let rows : Vec<&str> = input.lines().collect();
    let map_limit = rows.len();
    let mut antinode_map : Vec<Vec<char>> = vec![vec!['.'; map_limit]; map_limit];
    let mut antenna_map : HashMap<char, Vec<(usize, usize)>> = HashMap::new();
    
    for row in 0..map_limit {
        for column in 0..map_limit {
            let point = rows[row].chars().nth(column).unwrap();
            antinode_map[row][column] = point;
            if point.is_alphanumeric() {
                if antenna_map.get(&point).is_none(){
                    let mut antenna_list :Vec<(usize, usize)> = Vec::new();
                    antenna_list.push((row, column));
                    antenna_map.insert(point, antenna_list);
                } else {
                    let antenna_list: &mut Vec<(usize, usize)> = antenna_map.get_mut(&point).unwrap();
                    antenna_list.push((row, column));
                }
            }
        }
    }
    
    for (key, val) in antenna_map.iter() {
        // println!("key: {} value: {:?}", key, val);
        for point1 in 0..val.len() {
            for point2 in point1..val.len() {
                if point1 != point2 {
                    antinode_map[val[point1].0][val[point1].1] = '#';
                    antinode_map[val[point2].0][val[point2].1] = '#';
                    let distance_row = val[point2].0 as isize - val[point1].0 as isize;
                    let distance_column = val[point2].1 as isize - val[point1].1 as isize;
                    let mut subtract_counter = 1;
                    let mut add_counter = 1;
                    loop {
                        let antinode_1_row = val[point1].0.checked_add_signed(-1 * distance_row * subtract_counter);
                        let antinode_1_column = val[point1].1.checked_add_signed(-1 * distance_column * subtract_counter);
                        if antinode_1_row.is_some()
                            && antinode_1_row.unwrap() < map_limit 
                            && antinode_1_column.is_some() 
                            && antinode_1_column.unwrap() < map_limit {
                            antinode_map[antinode_1_row.unwrap()][antinode_1_column.unwrap()] = '#';
                            subtract_counter += 1;
                            // println!("antinode 1: {}, {}", antinode_1_row.unwrap(), antinode_1_column.unwrap());
                        } else {
                            break;
                        }
                    }
                    loop {
                        let antinode_2_row = val[point2].0.checked_add_signed(distance_row * add_counter);
                        let antinode_2_column = val[point2].1.checked_add_signed(distance_column * add_counter);
                        if antinode_2_row.is_some() 
                            && antinode_2_row.unwrap() < map_limit 
                            && antinode_2_column.is_some() 
                            && antinode_2_column.unwrap() < map_limit {
                            antinode_map[antinode_2_row.unwrap()][antinode_2_column.unwrap()] = '#';
                            // println!("antinode 2: {}, {}", antinode_2_row.unwrap(), antinode_2_column.unwrap());
                            add_counter += 1;
                        } else {
                            break;
                        }
                    }
                    // println!("point1: {:?}; point2: {:?}; distance_row: {}; distance_column: {}", val[point1], val[point2], distance_row, distance_column);
                    // for row in 0..antinode_map.len() {
                    //     for column in 0..antinode_map.len() {
                    //         print!("{}", antinode_map[row][column]);
                    //     }
                    //     println!();
                    // }
                }
            }
        }
    }
    
    let mut sum_antinode = 0;
    for row in 0..antinode_map.len() {
        for column in 0..antinode_map.len() {
            // print!("{}", antinode_map[row][column]);
            if antinode_map[row][column] == '#' {
                sum_antinode += 1;
            }
        }
        // println!();
    }
    Some(sum_antinode)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(14));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(34));
    }
}
