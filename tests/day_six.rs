extern crate aoc2019;


#[cfg(test)]
mod tests {
    use aoc2019::day_six::{
        create_hashmap_from_input,
        calculate_orbit_map_checksum,
        calculate_orbit_transfers
    };
    use std::fs::File;
    use std::io::Read;

    #[test]
    fn test_hashmap_create() {
        let input = String::from("COM)B\nB)C\nC)D\nD)E\nE)F\nB)G\nG)H\nD)I\nE)J\nJ)K\nK)L");
        let map = create_hashmap_from_input(input);
        assert_eq!(map.len(), 11);
    }

    #[test] 
    fn test_calculate_orbit_checksum() {
        let input = String::from("COM)B\nB)C\nC)D\nD)E\nE)F\nB)G\nG)H\nD)I\nE)J\nJ)K\nK)L");
        let checksum = calculate_orbit_map_checksum(input);
        assert_eq!(checksum, 42);
    }

    #[test]
    fn test_orbit_transfer_count() {
        let input = String::from("COM)B\nB)C\nC)D\nD)E\nE)F\nB)G\nG)H\nD)I\nE)J\nJ)K\nK)L\nK)YOU\nI)SAN\n");
        let smallest_distance = calculate_orbit_transfers(input);
        assert_eq!(smallest_distance, 4);
    }

    #[test]
    fn day_six_part_one() {
        let mut input = String::new();
        let mut file = File::open("./input/input6.txt").expect("Unable to open input file.");
        file.read_to_string(&mut input).expect("Unable to read file contents to string");

        let checksum = calculate_orbit_map_checksum(input);

        println!("{}", checksum);
    }

    #[test]
    fn day_six_part_two() {
        let mut input = String::new();
        let mut file = File::open("./input/input6.txt").expect("Unable to open input file.");
        file.read_to_string(&mut input).expect("Unable to read file contents to string");

        let distance = calculate_orbit_transfers(input);

        println!("{}", distance);
    }
}