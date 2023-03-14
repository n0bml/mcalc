use std::{io::{self, Write}, fmt};

type Error = Box<dyn std::error::Error>;
type Result<T> = std::result::Result<T, Error>;

enum WireUnits {
    Inches,
    Millimeters,
    Wavelengths,
}

impl fmt::Display for WireUnits {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            WireUnits::Inches => write!(f, "in"),
            WireUnits::Millimeters => write!(f, "mm"),
            WireUnits::Wavelengths => write!(f, "wavelengths"),
        }
    }
}

fn fmt_f64(num: f64, width: usize, precision: usize, exp_pad: usize) -> String {
    let mut num = format!("{:.precision$e}", num, precision = precision);
    let exp = num.split_off(num.find('e').unwrap());
    let (sign, exp) = if exp.starts_with("e-") {
        ('-', &exp[2..])
    } else {
        ('+', &exp[1..])
    };
    num.push_str(&format!("e{}{:0>pad$}", sign, exp, pad = exp_pad));
    format!("{:>width$}", num, width = width)
}

fn get_user_input(prompt: &str) -> String {
    print!("{}: ", prompt);
    io::stdout().flush().unwrap();
    let mut input: String = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input.trim().to_string()
}

fn get_frequency() -> Result<f64> {
    let fstr = get_user_input("Enter desired frequency in MHz");
    let parsed = fstr.parse::<f64>();
    match parsed {
        Ok(value) => Ok(value),
        Err(_) => Err(format!("Invalid value ('{}') for frequency!", fstr).into()),
    }
    // TODO check for negative, or too low, values
}

fn get_units() -> Result<WireUnits> {
    println!("\nSelect units for wire diameter in:");
    println!("    1. Inches");
    println!("    2. Millimeters");
    println!("    3. Wavelengths");
    let ustr = get_user_input("Choose 1, 2, or 3");
    let value = ustr.parse::<isize>();
    match value {
        Ok(value) => value,
        Err(_) => return Err(format!("Invalid number ('{}') for wire diamter unit choice.", ustr).into()),
    };

    match value {
        Ok(1) => Ok(WireUnits::Inches),
        Ok(2) => Ok(WireUnits::Millimeters),
        Ok(3) => Ok(WireUnits::Wavelengths),
        _ => Err(format!("Invalid choice ({}) for wire diameter units.", ustr).into())
    }
}

// returns the chosen wire diameter in wavelengths
fn get_wire_diameter(units: &WireUnits) -> Result<f64> {
    let wds = get_user_input(&format!("Enter wire diameter in {}", units));
    let parsed = wds.parse::<f64>();
    let wd = match parsed {
        Ok(value) => value,
        Err(_) => return Err(format!("Invalid value ('{}') for wire diameter!", wds).into()),
    };
    // TODO check for negative numbers

    let wli: f64 = match units {
        WireUnits::Inches => 11802.71,
        WireUnits::Millimeters => 299792.5,
        WireUnits::Wavelengths => 1.0,
    };

    let dw = wd / wli;
    println!("\nWire diameter in wavelengths: {}", fmt_f64(dw, 0, 4, 2));

    Ok(dw)
}

fn main() -> Result<()> {
    println!("Program to calculate the dimensions of a Moxon Rectangle.");
    println!("All equations correlated to NEC antenna modeling software for wire diameters");
    println!("     from 1E-5 to 1E-2 wavelengths.");
    println!("L. B. Cebik, W4RNL (SK)");
    println!("Converted to Rust by Brendan Leber, N0BML\n");

    let frequency = match get_frequency() {
        Ok(f) => f,
        Err(e) => return Err(e),
    };

    let units = match get_units() {
        Ok(u) => u,
        Err(e) => return Err(e),
    };

    let wire_diameter = match get_wire_diameter(&units) {
        Ok(wd) => wd,
        Err(e) => return Err(e),
    };

    let d1 = std::f64::consts::LOG10_E * wire_diameter.ln();
    if d1 < -6.0 {
        println!("Wire diameter less than 1E-6 wavelengths; results uncertain.");
    } else if d1 > -2.0 {
        println!("Wire diameter greater than 1E-2 wavelengths; results uncertain.");
    }

    let aa = -0.0008571428571;
    let ab = -0.009571428571;
    let ac =  0.3398571429;
    let a = (aa * d1.powf(2.0)) + (ab * d1) + ac;

    let ba = -0.002142857143;
    let bb = -0.02035714286;
    let bc =  0.008285714286;
    let b = (ba * d1.powf(2.0)) + (bb * d1) + bc;

    let ca = 0.001809523381;
    let cb = 0.01780952381;
    let cc = 0.05164285714;
    let c = (ca * d1.powf(2.0)) + (cb * d1) + cc;

    let da = 0.001;
    let db = 0.07178571429;
    let d = (da * d1) +db;

    let e = (b + c) + d;

    println!("\nMoxon Dimensions in Wavelengths:");
    println!("    A = {}", fmt_f64(a, 0, 4, 2));
    println!("    B = {}", fmt_f64(b, 0, 4, 2));
    println!("    C = {}", fmt_f64(c, 0, 4, 2));
    println!("    D = {}", fmt_f64(d, 0, 4, 2));
    println!("    E = {}", fmt_f64(e, 0, 4, 2));

    let wf = 983.5592 / frequency;
    let wfi = wf * 12.0;
    println!("\nWavelength: {:.4} feet or {:.2} inches.", wf, wfi);

    println!("\nDimensions in Feet and Inches:");
    println!("    A = {:.4} feet or {:.2} inches.", a * wf, a * wfi);
    println!("    B = {:.4} feet or {:.2} inches.", b * wf, b * wfi);
    println!("    C = {:.4} feet or {:.2} inches.", c * wf, c * wfi);
    println!("    D = {:.4} feet or {:.2} inches.", d * wf, d * wfi);
    println!("    E = {:.4} feet or {:.2} inches.", e * wf, e * wfi);

    Ok(())
}
