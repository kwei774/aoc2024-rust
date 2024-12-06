advent_of_code::solution!(6);

pub fn part_one(input: &str) -> Option<u32> {
    let rows : Vec<&str> = input.lines().collect();
    let mut map : Vec<Vec<char>> = vec![Vec::new(); rows.len()];
    for row in 0..rows.len() {
        map[row].extend(rows[row].chars());
    }
    let mut current_row = 0;
    let mut current_column = 0;
    let mut direction = '^';
    let mut found_start = false;
    for row in 0..map.len() {
        for column in 0..map[row].len() {
            if map[row][column] == '^'{
                current_row = row;
                current_column = column;
                found_start = true;
                break;
            }
        }
        if found_start {
            break;
        }
    }
    // println!("original:");
    // for row in 0..map.len() {
    //     for column in 0..map[row].len() {
    //         print!("{}", map[row][column]);
    //     }
    //     println!();
    // }
    loop {
        map[current_row][current_column] = 'X';
        if direction == '^' {
            if current_row == 0 {
                break;
            } else if map[current_row - 1][current_column] == '#' {
                direction = '>';
            } else {
                current_row -= 1;
            }
        } else if direction == '>' {
            if current_column == map[current_column].len() - 1 {
                break;
            } else if map[current_row][current_column + 1] == '#' {
                direction = 'V';
            } else {
                current_column += 1;
            }
        } else if direction == 'V' {
            if current_row == map.len() - 1 {
                break;
            } else if map[current_row + 1][current_column] == '#' {
                direction = '<';
            } else {
                current_row += 1;
            }
        } else {
            if current_column == 0 {
                break;
            } else if map[current_row][current_column - 1] == '#' {
                direction = '^';
            } else {
                current_column -= 1;
            }
        }
    }
    // println!("solution:");
    let mut path = 0;
    for row in 0..map.len() {
        for column in 0..map[row].len() {
            // print!("{}", map[row][column]);
            if map[row][column] == 'X'{
                path += 1;
            }
        }
        // println!();
    }
    Some(path)
}

pub fn part_two(input: &str) -> Option<u32> {
    let rows : Vec<&str> = input.lines().collect();
    let mut map : Vec<Vec<char>> = vec![Vec::new(); rows.len()];
    for row in 0..rows.len() {
        map[row].extend(rows[row].chars());
    }
    let mut start_row = 0;
    let mut start_column = 0;
    let mut direction = '^';
    let mut found_start = false;
    for row in 0..map.len() {
        for column in 0..map[row].len() {
            if map[row][column] == '^'{
                start_row = row;
                start_column = column;
                found_start = true;
                break;
            }
        }
        if found_start {
            break;
        }
    }
    let mut forever_loops = 0;
    for row in 0..map.len() {
        for column in 0..map[row].len() {
            if map[row][column] == '.' {
                let mut time_warp = map.clone();
                let mut current_row = start_row;
                let mut current_column = start_column;
                let mut current_direction = direction;
                time_warp[row][column] = '#';

                // println!("time_warp beginning:");
                // for time_warp_row in 0..time_warp.len() {
                //     for time_warp_column in 0..time_warp[time_warp_row].len() {
                //         print!("{}", time_warp[time_warp_row][time_warp_column]);
                //     }
                //     println!();
                // }
                let mut brute_force = time_warp.len() * time_warp[0].len();
                while brute_force > 0 {
                    time_warp[current_row][current_column] = 'X';
                    if current_direction == '^' {
                        if current_row == 0 {
                            break;
                        } else if time_warp[current_row - 1][current_column] == '#' {
                            current_direction = '>';
                        } else {
                            current_row -= 1;
                        }
                    } else if current_direction == '>' {
                        if current_column == time_warp[current_column].len() - 1 {
                            break;
                        } else if time_warp[current_row][current_column + 1] == '#' {
                            current_direction = 'V';
                        } else {
                            current_column += 1;
                        }
                    } else if current_direction == 'V' {
                        if current_row == time_warp.len() - 1 {
                            break;
                        } else if time_warp[current_row + 1][current_column] == '#' {
                            current_direction = '<';
                        } else {
                            current_row += 1;
                        }
                    } else {
                        if current_column == 0 {
                            break;
                        } else if time_warp[current_row][current_column - 1] == '#' {
                            current_direction = '^';
                        } else {
                            current_column -= 1;
                        }
                    }
                    brute_force -= 1;
                }
                // println!("time_warp end:");
                // for time_warp_row in 0..time_warp.len() {
                //     for time_warp_column in 0..time_warp[time_warp_row].len() {
                //         print!("{}", time_warp[time_warp_row][time_warp_column]);
                //     }
                //     println!();
                // }
                if brute_force == 0{
                    forever_loops += 1;
                }
            }
        }
    }
    Some(forever_loops)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(41));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(6));
    }
}
