advent_of_code::solution!(9);

pub fn part_one(input: &str) -> Option<isize> {
    let mut disk_map: Vec<isize> = Vec::new();
    let mut chars = input.chars();
    let mut file_counter = 0;
    loop {
        let file = chars.next();
        if file.is_some() {
            for _i in 0..file.unwrap().to_digit(10).unwrap() {
                disk_map.push(file_counter);
            }
            file_counter += 1;
        } else {
            break;
        }
        let free = chars.next();
        if free.is_some() {
            for _i in 0..free.unwrap().to_digit(10).unwrap() {
                disk_map.push(-1);
            }
        } else {
            break;
        }
    }
    // let print_1 = disk_map.clone();
    // for identity in print_1 {
    //     if identity == -1 {
    //         print!(".");
    //     } else {
    //         print!("{}", identity);
    //     }
    // }
    // println!();

    for check_file in (0..disk_map.len()).rev() {
        if disk_map[check_file] != -1 {
            for first_empty_file in 0..disk_map.len() {
                if disk_map[first_empty_file] == -1 && first_empty_file < check_file{
                    disk_map[first_empty_file] = disk_map[check_file];
                    disk_map[check_file] = -1;
                    break;
                }
            }
        }
        // let print_x = disk_map.clone();
        // for identity in print_x {
        //     if identity == -1 {
        //         print!(".");
        //     } else {
        //         print!("{}", identity);
        //     }
        // }
        // println!();
    }
    let mut checksum = 0;
    for position in 0..disk_map.len() {
        if disk_map[position] != -1 {
            checksum += position as isize * disk_map[position];
        }
    }

    Some(checksum)
}

pub fn part_two(input: &str) -> Option<isize> {
    let mut disk_map: Vec<isize> = Vec::new();
    let mut chars = input.chars();
    let mut file_counter = 0;
    loop {
        let file = chars.next();
        if file.is_some() {
            for _i in 0..file.unwrap().to_digit(10).unwrap() {
                disk_map.push(file_counter);
            }
            file_counter += 1;
        } else {
            break;
        }
        let free = chars.next();
        if free.is_some() {
            for _i in 0..free.unwrap().to_digit(10).unwrap() {
                disk_map.push(-1);
            }
        } else {
            break;
        }
    }
    // let print_1 = disk_map.clone();
    // for identity in print_1 {
    //     if identity == -1 {
    //         print!(".");
    //     } else {
    //         print!("{}", identity);
    //     }
    // }
    // println!();
    file_counter -= 1;

    for check_file_end in (0..disk_map.len()).rev() {
        if disk_map[check_file_end] != -1 && disk_map[check_file_end] == file_counter{
            let mut check_file_beg = 0;
            for find_beginning in 0..disk_map.len() {
                if disk_map[find_beginning] == disk_map[check_file_end]{
                    check_file_beg = find_beginning;
                    break;
                }
            }
            let file_size = check_file_end - check_file_beg + 1;
            let mut empty_size = 0;
            file_counter -= 1;
            for first_empty_space in 0..disk_map.len() {
                let mut end_empty_space = 0;
                if disk_map[first_empty_space] == -1 {
                    for find_end in first_empty_space..disk_map.len() {
                        if disk_map[find_end] != -1 {
                            end_empty_space = find_end;
                            break;
                        }
                    }
                    // println!("{}, {}", end_empty_space, first_empty_space);
                    empty_size = end_empty_space as isize - first_empty_space as isize;
                }
                if file_size as isize <= empty_size && first_empty_space < check_file_beg {
                    for replace_empty in first_empty_space..first_empty_space + file_size {
                        disk_map[replace_empty] = disk_map[check_file_end];
                    }
                    for replace_file in check_file_beg..check_file_end + 1 {
                        disk_map[replace_file] = -1;
                    }
                    // let print_x = disk_map.clone();
                    // for identity in print_x {
                    //     if identity == -1 {
                    //         print!(".");
                    //     } else {
                    //         print!("{}", identity);
                    //     }
                    // }
                    // println!();
                    break;
                }
            }
        }
    }
    let mut checksum = 0;
    for position in 0..disk_map.len() {
        if disk_map[position] != -1 {
            checksum += position as isize * disk_map[position];
        }
    }

    Some(checksum)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1928));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2858));
    }
}
