use std::collections::{HashMap, HashSet};

pub fn create_hashmap_from_input(input: String) -> HashMap<String, String> {
    let mut map: HashMap<String, String> = HashMap::new();
    let input_vec: Vec<&str> = input.trim().split("\n").collect();
    for s in input_vec {
        let orbit_kv: Vec<&str> = s.split(")").collect();
        map.insert(String::from(orbit_kv[1]), String::from(orbit_kv[0]));
    }
    map
}

pub fn calculate_orbit_map_checksum(input: String) -> u32 {
    let mut seen_orbits: HashSet<(&String, &String)> = HashSet::new();

    let orbit_hashmap = create_hashmap_from_input(input);

    let hashmap_keys = orbit_hashmap.keys();

    for key in hashmap_keys {
        let mut next_node = orbit_hashmap.get(key).unwrap();
        seen_orbits.insert((key, next_node));
        while next_node != "COM" {
            next_node = orbit_hashmap.get(next_node).unwrap();
            seen_orbits.insert((key, next_node));
        }
    }

    seen_orbits.len() as u32
}

fn distance_between_orbits(orbits: &HashMap<String, String>, orbit_start: &String, orbit_end: &String) -> i32 {
    let mut orbit = orbits.get(orbit_start).unwrap();
    let mut total_orbits = 0;
    while orbit != orbit_end {
        orbit = orbits.get(orbit).unwrap();
        total_orbits += 1;
    }
    total_orbits
}

pub fn calculate_orbit_transfers(input: String) -> i32 {
    let orbit_hashmap = create_hashmap_from_input(input);

    let mut you_orbiting = orbit_hashmap.get("YOU").unwrap();
    let mut san_orbiting = orbit_hashmap.get("SAN").unwrap();

    let mut you_orbit_map_to_com: HashSet<&String> = HashSet::new();
    let mut san_orbit_map_to_com: HashSet<&String> = HashSet::new();

    you_orbit_map_to_com.insert(you_orbiting);
    san_orbit_map_to_com.insert(san_orbiting);

    while you_orbiting != "COM" {
        you_orbiting = orbit_hashmap.get(you_orbiting).unwrap();
        you_orbit_map_to_com.insert(you_orbiting);
    }

    while san_orbiting != "COM" {
        san_orbiting = orbit_hashmap.get(san_orbiting).unwrap();
        san_orbit_map_to_com.insert(san_orbiting);
    }

    let orbit_intersections = san_orbit_map_to_com.intersection(&you_orbit_map_to_com);

    let mut smallest_distance: i32 = -1;
    for orbit in orbit_intersections {
        let san_to_orbit = distance_between_orbits(&orbit_hashmap, &String::from("SAN"), orbit);
        let you_to_orbit = distance_between_orbits(&orbit_hashmap, &String::from("YOU"), orbit);

        if smallest_distance == -1 || smallest_distance > san_to_orbit + you_to_orbit {
            smallest_distance = san_to_orbit + you_to_orbit;
        }
    }

    smallest_distance
}