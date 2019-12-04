extern crate aoc2019;

#[cfg(test)]
mod tests {

    #[test]
    fn test_get_distance_between_closest_intersection_and_start() {
        use aoc2019::day_three::get_distance_between_closest_intersect_and_start;

        let test_case_one_wire_one = "R8,U5,L5,D3";
        let test_case_one_wire_two = "U7,R6,D4,L4";

        let test_case_two_wire_one = "R75,D30,R83,U83,L12,D49,R71,U7,L72";
        let test_case_two_wire_two = "U62,R66,U55,R34,D71,R55,D58,R83";

        let test_case_three_wire_one = "R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51";
        let test_case_three_wire_two = "U98,R91,D20,R16,D67,R40,U7,R15,U6,R7";

        let test_case_one_distance = get_distance_between_closest_intersect_and_start(test_case_one_wire_one, test_case_one_wire_two);
        let test_case_two_distance = get_distance_between_closest_intersect_and_start(test_case_two_wire_one, test_case_two_wire_two);
        let test_case_three_distance = get_distance_between_closest_intersect_and_start(test_case_three_wire_one, test_case_three_wire_two);

        assert_eq!(test_case_one_distance, 6);
        assert_eq!(test_case_two_distance, 159);
        assert_eq!(test_case_three_distance, 135);
    }

    #[test]
    fn day_three_part_one() {
        use aoc2019::day_three::get_distance_between_closest_intersect_and_start;
        use std::fs::File;
        use std::io::Read;

        let mut input = String::new();
        let mut file = File::open("./input/input3.txt").expect("Unable to find input file.");
        file.read_to_string(&mut input).expect("Unable to read file contents into string.");

        let wires: Vec<&str> = input.split("\n").collect();

        let distance = get_distance_between_closest_intersect_and_start(wires[0], wires[1]);

        println!("Smallest Manhatten Distance from central port - {}", distance);
    }
}