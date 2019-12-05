use std::collections::{HashSet, HashMap};

pub fn get_distance_between_closest_intersect_and_start_and_shortest_steps(wire_one: &str, wire_two: &str) -> (i32, i32) {
    let mut wire_one_coords: HashSet<(i32, i32)> = HashSet::new();
    let mut wire_two_coords: HashSet<(i32, i32)> = HashSet::new();
    let mut wire_one_steps: HashMap<(i32, i32), i32> = HashMap::new();
    let mut wire_two_steps: HashMap<(i32, i32), i32> = HashMap::new();

    let mut current_coordinates: (i32, i32) = (0, 0);
    let mut steps = 0;
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
            steps += 1;
            wire_one_coords.insert(current_coordinates);
            if !wire_one_steps.contains_key(&current_coordinates) {
                wire_one_steps.insert(current_coordinates, steps);
            }
        }
    }

    current_coordinates = (0, 0);
    steps = 0;
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
            steps += 1;
            wire_two_coords.insert(current_coordinates);
            if !wire_two_steps.contains_key(&current_coordinates) {
                wire_two_steps.insert(current_coordinates, steps);
            }
        }
    }

    let intersection = wire_one_coords.intersection(&wire_two_coords);
    
    let mut smallest_manhattan_distance = -1;
    let mut smallest_amount_of_steps = -1;

    for coord in intersection {
        let distance = coord.0.abs() + coord.1.abs();
        let steps = wire_one_steps.get(coord).unwrap() + wire_two_steps.get(coord).unwrap();
        if smallest_manhattan_distance < 0 {
            smallest_manhattan_distance = distance;
        } else if distance < smallest_manhattan_distance {
            smallest_manhattan_distance = distance;
        }

        if smallest_amount_of_steps < 0 {
            smallest_amount_of_steps = steps;
        } else if steps < smallest_amount_of_steps {
            smallest_amount_of_steps = steps;
        }
    }

    (smallest_manhattan_distance, smallest_amount_of_steps)
}