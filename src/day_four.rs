pub fn find_possible_password_combination_count(lower_bound: u32, upper_bound: u32) -> u32 {
    let mut total_password_combos = 0;

    for i in lower_bound..upper_bound {
        if has_adjacent_numbers(i) && numbers_increase_left_to_right(i) {
            total_password_combos += 1;
        }
    }

    total_password_combos
}

pub fn find_possible_password_combination_count_part_two(lower_bound: u32, upper_bound: u32) -> u32 {
    let mut total_password_combos = 0;

    for i in lower_bound..upper_bound {
        if has_two_adjacent_numbers_not_in_larger_group(i) && numbers_increase_left_to_right(i) {
            total_password_combos += 1;
        }
    }

    total_password_combos
}

pub fn has_adjacent_numbers(num: u32) -> bool {
    let num_string = num.to_string();
    let num_string_bytes = num_string.as_bytes();
    num_string_bytes[0] == num_string_bytes[1] ||
    num_string_bytes[1] == num_string_bytes[2] ||
    num_string_bytes[2] == num_string_bytes[3] ||
    num_string_bytes[3] == num_string_bytes[4] ||
    num_string_bytes[4] == num_string_bytes[5]
}

pub fn has_two_adjacent_numbers_not_in_larger_group(num: u32) -> bool {
    let num_string = num.to_string();
    let num_string_bytes = num_string.as_bytes();
    (num_string_bytes[0] == num_string_bytes[1] && num_string_bytes[1] != num_string_bytes[2]) ||
    (num_string_bytes[0] != num_string_bytes[1] && num_string_bytes[1] == num_string_bytes[2] && num_string_bytes[2] != num_string_bytes[3]) ||
    (num_string_bytes[1] != num_string_bytes[2] && num_string_bytes[2] == num_string_bytes[3] && num_string_bytes[3] != num_string_bytes[4]) ||
    (num_string_bytes[2] != num_string_bytes[3] && num_string_bytes[3] == num_string_bytes[4] && num_string_bytes[4] != num_string_bytes[5]) ||
    (num_string_bytes[3] != num_string_bytes[4] && num_string_bytes[4] == num_string_bytes[5])
}

pub fn numbers_increase_left_to_right(num: u32) -> bool {
    let num_string = num.to_string();
    let num_string_bytes = num_string.as_bytes();
    num_string_bytes[0] <= num_string_bytes[1] &&
    num_string_bytes[1] <= num_string_bytes[2] &&
    num_string_bytes[2] <= num_string_bytes[3] &&
    num_string_bytes[3] <= num_string_bytes[4] &&
    num_string_bytes[4] <= num_string_bytes[5]
}