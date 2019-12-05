extern crate aoc2019;

#[cfg(test)]
mod tests {
    #[test]
    fn has_adjacent_numbers_should_be_false() {
        use aoc2019::day_four::has_adjacent_numbers;
        let n = 123456;
        assert_eq!(has_adjacent_numbers(n), false);
    
    }
    #[test]
    fn has_adjacent_numbers_should_be_true() {
        use aoc2019::day_four::has_adjacent_numbers;
        let n = 113456;
        assert_eq!(has_adjacent_numbers(n), true);
    }

    #[test]
    fn numbers_increase_left_to_right_is_false() {
        use aoc2019::day_four::numbers_increase_left_to_right;
        let n = 123321;
        assert_eq!(numbers_increase_left_to_right(n), false);
    }

    #[test]
    fn numbers_increase_left_to_right_is_true() {
        use aoc2019::day_four::numbers_increase_left_to_right;
        let n = 123456;
        let m = 111111;
        assert_eq!(numbers_increase_left_to_right(n), true);
        assert_eq!(numbers_increase_left_to_right(m), true);
    }

    #[test]
    fn test_has_two_adjacent_numbers_not_in_larger_group() {
        use aoc2019::day_four::has_two_adjacent_numbers_not_in_larger_group;
        let test_one = 112233;
        let test_two = 123444;
        let test_three = 111122;

        assert_eq!(has_two_adjacent_numbers_not_in_larger_group(test_one), true);
        assert_eq!(has_two_adjacent_numbers_not_in_larger_group(test_two), false);
        assert_eq!(has_two_adjacent_numbers_not_in_larger_group(test_three), true);    
    }

    #[test]
    fn day_four_part_one() {
        use aoc2019::day_four::find_possible_password_combination_count;
        let lower = 125730;
        let upper = 579381;

        let password_combo_count = find_possible_password_combination_count(lower, upper);

        println!("Password Combination Count - {}", password_combo_count);
    }

    #[test]
    fn day_four_part_two() {
        use aoc2019::day_four::find_possible_password_combination_count_part_two;
        let lower = 125730;
        let upper = 579381;

        let password_combo_count = find_possible_password_combination_count_part_two(lower, upper);

        println!("Password Combination Count Part 2 - {}", password_combo_count);
    }
}