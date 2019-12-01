pub fn get_required_fuel_for_module_mass(module_mass: f32) -> f32 {
    (module_mass / 3.0).floor() - 2.0
}

pub fn get_total_fuel_for_module_masses(module_masses: Vec<f32>) -> f32 {
    let mut total_mass: f32 = 0.0;
    for mass in module_masses {
        total_mass += get_required_fuel_for_module_mass(mass);
    }
    total_mass
}

pub fn get_required_fuel_for_module_mass_two(module_mass: f32) -> f32 {
    let mut total_required_fuel_for_module = 0.0;
    let mut required_fuel = get_required_fuel_for_module_mass(module_mass);

    while required_fuel > 0.0 {
        total_required_fuel_for_module += required_fuel;
        required_fuel = get_required_fuel_for_module_mass(required_fuel);
    }
    total_required_fuel_for_module
}

pub fn get_total_fuel_for_module_masses_two(module_masses: Vec<f32>) -> f32 {
    let mut total_mass: f32 = 0.0;
    for mass in module_masses {
        total_mass += get_required_fuel_for_module_mass_two(mass);
    }
    total_mass
}
