pub mod day_one;

#[cfg(test)]
mod tests {
    const MODULE_ONE_MASS: f32 = 12.0;
    const MODULE_TWO_MASS: f32 = 14.0;
    const MODULE_THREE_MASS: f32 = 1969.0;
    const MODULE_FOUR_MASS: f32 = 100756.0;

    #[test]  
    fn get_required_fuel_for_module_mass_success() {
        use super::day_one::get_required_fuel_for_module_mass;
        let expected_module_one_fuel = 2.0;
        let expected_module_two_fuel = 2.0;
        let expected_module_three_fuel = 654.0;
        let expected_module_four_fuel = 33583.0;
        assert_eq!(
            get_required_fuel_for_module_mass(MODULE_ONE_MASS), 
            expected_module_one_fuel
        );
        assert_eq!(
            get_required_fuel_for_module_mass(MODULE_TWO_MASS), 
            expected_module_two_fuel
        );
        assert_eq!(
            get_required_fuel_for_module_mass(MODULE_THREE_MASS), 
            expected_module_three_fuel
        );
        assert_eq!(
            get_required_fuel_for_module_mass(MODULE_FOUR_MASS), 
            expected_module_four_fuel
        );
    }

    #[test] 
    fn get_total_fuel_for_module_masses_success() {
        use super::day_one::get_total_fuel_for_module_masses;
        let module_masses = vec![12.0, 14.0, 1969.0, 100756.0];
        let expected_total_mass = 34241.0;

        assert_eq!(get_total_fuel_for_module_masses(module_masses), expected_total_mass);
    }

    #[test]
    fn get_required_fuel_for_module_mass_two_success() {
        use super::day_one::get_required_fuel_for_module_mass_two;
        let expected_module_one_fuel = 2.0;
        let expected_module_three_fuel = 966.0;
        let expected_module_four_fuel = 50346.0;

        assert_eq!(
            get_required_fuel_for_module_mass_two(MODULE_ONE_MASS), 
            expected_module_one_fuel
        );
        assert_eq!(
            get_required_fuel_for_module_mass_two(MODULE_THREE_MASS), 
            expected_module_three_fuel
        );
        assert_eq!(
            get_required_fuel_for_module_mass_two(MODULE_FOUR_MASS), 
            expected_module_four_fuel
        );
    }

    #[test]
    fn day_one_part_one_solution() {
        use super::day_one::get_total_fuel_for_module_masses;
        use std::fs::File;
        use std::io::Read;

        let mut input_one = String::new();
        let mut file = File::open("./input/input1.txt").expect("Unable to find input file.");
        file.read_to_string(&mut input_one).expect("Unable to read file contents into string.");
        
        let masses = input_one.trim().split("\n").map(|x| x.parse::<f32>().unwrap()).collect::<Vec<f32>>();
        let total_fuel = get_total_fuel_for_module_masses(masses);
        println!("Total Fuel Part 1 - {}", total_fuel);
    }

    #[test]
    fn day_one_part_two_solution() {
        use super::day_one::get_total_fuel_for_module_masses_two;
        use std::fs::File;
        use std::io::Read;

        let mut input_one = String::new();
        let mut file = File::open("./input/input1.txt").expect("Unable to find input file.");
        file.read_to_string(&mut input_one).expect("Unable to read file contents into string.");
        
        let masses = input_one.trim().split("\n").map(|x| x.parse::<f32>().unwrap()).collect::<Vec<f32>>();
        let total_fuel = get_total_fuel_for_module_masses_two(masses);
        println!("Total Fuel Part 2 - {}", total_fuel);
    }
}
