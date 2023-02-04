use std::io;

fn get_frequency() -> f64 {
    let mut input: String = String::new();

    print!("Enter desired frequency in MHz: ");
    match io::stdin().read_line(&mut input) {
        Ok(_) => {
            input = input.trim().to_string();
        },
        Err(e) => {
            println!("Error reading input: {}", e);
            panic!();  // TODO - handle the Rust way
        },
    }

    let frequency: f64 = match input.parse::<f64>() {
        Ok(f) =>  f,
        Err(e) => {
            println!("Unable to parse '{}' as a floating point number.\n{}", input, e);
            panic!();  // TODO - handle the Rust way
        },
    };

    frequency
}

fn get_units() -> isize {
    let mut input: String = String::new();

    println!("\nSelect units for wire diameter in:");
    println!("    1. Inches");
    println!("    2. Millimeters");
    println!("    3. Wavelengths");
    print!("Choose 1, 2, or 3: ");

    match io::stdin().read_line(&mut input) {
        Ok(_) => {
            input = input.trim().to_string();
        },
        Err(e) => {
            println!("Error reading input: {}", e);
            panic!();  // TODO - handle the Rust way
        },
    };

    let units: isize = match input.as_str() {
        "1" => 1,
        "2" => 2,
        "3" => 3,
        _ => {
            println!("Invalid choice: {}", input);
            panic!();  // TODO - handle the Rust way
        }
    };

    units
}

fn get_wire_diameter(units: &isize) -> f64 {
    let mut input: String = String::new();

    let units_name: &str = match units {
        1 => "inches",
        2 => "millimeters",
        3 => "wavelengths",
        _ => panic!(),
    };

    print!("Enter wire diameter in {}: ", units_name);

    match io::stdin().read_line(&mut input) {
        Ok(_) => {
            input = input.trim().to_string();
        },
        Err(e) => {
            println!("Error reading input: {}", e);
            panic!();  // TODO
        },
    };

    let mut dw: f64 = 0.0;

    dw
}

fn main() {
    println!("Program to calculate the dimensions of a Moxon Rectangle.");
    println!("All equations correlated to NEC antenna modeling software for wire diameters");
    println!("     from 1E-5 to 1E-2 wavelengths.");
    println!("L. B. Cebik, W4RNL (SK)");
    println!("Converted to Rust by Brendan Leber, N0BML\n");

    let frequency = get_frequency();
    let units = get_units();
    let wire_diameter = get_wire_diameter(&units);

}
