use std::collections::HashSet;

pub fn get_distance_between_closest_intersect_and_start(wire_one: &str, wire_two: &str) -> i32 {
    let mut wire_one_coords: HashSet<(i32, i32)> = HashSet::new();
    let mut wire_two_coords: HashSet<(i32, i32)> = HashSet::new();

    let mut current_coordinates: (i32, i32) = (0, 0);

    for line in wire_one.split(",") {
        let direction = line.get(0..1).unwrap();
        let length_str = line.get(1..).unwrap();
        let length = length_str.parse::<u32>().unwrap();

        for _ in 0..length {
            match direction {
                "D" => current_coordinates.1 += 1,
                "U" => current_coordinates.1 -= 1,
                "R" => current_coordinates.0 += 1,
                "L" => current_coordinates.0 -= 1,
                _ => panic!("Unknown direction value encountered!")
            }
            wire_one_coords.insert(current_coordinates);
        }
    }

    current_coordinates = (0, 0);

    for line in wire_two.split(",") {
        let direction = line.get(0..1).unwrap();
        let length_str = line.get(1..).unwrap();
        let length = length_str.parse::<u32>().unwrap();

        for _ in 0..length {
            match direction {
                "D" => current_coordinates.1 += 1,
                "U" => current_coordinates.1 -= 1,
                "R" => current_coordinates.0 += 1,
                "L" => current_coordinates.0 -= 1,
                _ => panic!("Unknown direction value encountered!")
            }
            wire_two_coords.insert(current_coordinates);
        }
    }

    let intersection = wire_one_coords.intersection(&wire_two_coords);
    
    let mut smallest_manhattan_distance = -1;

    for coord in intersection {
        let distance = coord.0.abs() + coord.1.abs();
        if smallest_manhattan_distance < 0 {
            smallest_manhattan_distance = distance;
        } else if distance < smallest_manhattan_distance {
            smallest_manhattan_distance = distance;
        }
    }

    smallest_manhattan_distance
}