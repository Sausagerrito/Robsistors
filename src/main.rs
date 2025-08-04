use std::fmt;

fn main() {
    println!(
        "blac[K] brow[N] [R]ed [O]range [Y]ellow [G]reen b[L]ue [V]iolet gr[E]y [W]hite gol[D] [S]ilver"
    );
    let mut v: Vec<char> = vec![];
    v.push('R'.to_ascii_lowercase());
    v.push('R'.to_ascii_lowercase());
    v.push('R'.to_ascii_lowercase());
    v.push('R'.to_ascii_lowercase());

    let d: Bands = banding(v).expect("idk yet"); //data

    let cd: (f32, char, f32, char) = convert(d.hundreds, d.tens, d.ones, d.mult, d.tolerance); //converted data

    let r = Resistor {
        prefix: cd.0,
        suffix: cd.1,
        tolerance_prefix: cd.2,
        tolerance_suffix: cd.3,
        temp: d.temp_coef,
    };

    println!("{}", r);
}

#[derive(Clone, Copy, Debug)]
struct Resistor {
    prefix: f32,
    suffix: char,
    tolerance_prefix: f32,
    tolerance_suffix: char,
    temp: u8,
}

#[derive(Clone, Copy, Debug)]
struct Bands {
    hundreds: u8,
    tens: u8,
    ones: u8,
    mult: f32,
    tolerance: f32,
    temp_coef: u8,
}

impl fmt::Display for Resistor {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{} {}Ω ± {} {}Ω, {}ppm/C",
            self.prefix, self.suffix, self.tolerance_prefix, self.tolerance_suffix, self.temp
        )
    }
}

fn banding(v: Vec<char>) -> Result<Bands, Box<dyn std::error::Error>> {
    let size: usize = v.len();

    let mut raw = Bands {
        hundreds: 0,
        tens: 0,
        ones: 0,
        mult: 0.,
        tolerance: 0.,
        temp_coef: 0,
    };

    match size {
        3 => {
            raw.tens = prefix_map(v[0]).ok_or("character 1 invalid")?;
            raw.ones = prefix_map(v[1]).ok_or("character 2 invalid")?;
            raw.mult = suffix_map(v[2]).ok_or("character 3 invalid")?;
        }
        4 => {
            raw.tens = prefix_map(v[0]).ok_or("character 1 invalid")?;
            raw.ones = prefix_map(v[1]).ok_or("character 2 invalid")?;
            raw.mult = suffix_map(v[2]).ok_or("character 3 invalid")?;
            raw.tolerance = tolerance_map(v[3]).ok_or("character 4 invalid")?;
        }
        5 => {
            raw.hundreds = prefix_map(v[0]).ok_or("character 1 invalid")?;
            raw.tens = prefix_map(v[1]).ok_or("character 2 invalid")?;
            raw.ones = prefix_map(v[2]).ok_or("character 3 invalid")?;
            raw.mult = suffix_map(v[3]).ok_or("character 4 invalid")?;
            raw.tolerance = tolerance_map(v[4]).ok_or("character 5 invalid")?;
        }
        6 => {
            raw.hundreds = prefix_map(v[0]).ok_or("character 1 invalid")?;
            raw.tens = prefix_map(v[1]).ok_or("character 2 invalid")?;
            raw.ones = prefix_map(v[2]).ok_or("character 3 invalid")?;
            raw.mult = suffix_map(v[3]).ok_or("character 4 invalid")?;
            raw.tolerance = tolerance_map(v[4]).ok_or("character 5 invalid")?;
            raw.temp_coef = temp_map(v[5]).ok_or("character 6 invalid")?;
        }
        _ => return Err("unsupported length".into()),
    }
    Ok(raw)
}

fn prefix_map(c: char) -> Option<u8> {
    match c {
        'k' => Some(0),
        'n' => Some(1),
        'r' => Some(2),
        'o' => Some(3),
        'y' => Some(4),
        'g' => Some(5),
        'l' => Some(6),
        'v' => Some(7),
        'e' => Some(8),
        'w' => Some(9),
        _ => None,
    }
}

fn suffix_map(c: char) -> Option<f32> {
    match c {
        'k' => Some(1.),
        'n' => Some(10.),
        'r' => Some(100.),
        'o' => Some(1000.),
        'y' => Some(10000.),
        'g' => Some(100000.),
        'l' => Some(1000000.),
        'v' => Some(10000000.),
        'e' => Some(100000000.),
        'w' => Some(1000000000.),
        'd' => Some(0.1),
        's' => Some(0.01),
        _ => None,
    }
}

fn tolerance_map(c: char) -> Option<f32> {
    match c {
        'n' => Some(1.),
        'r' => Some(2.),
        'o' => Some(3.),
        'y' => Some(4.),
        'g' => Some(0.5),
        'l' => Some(0.25),
        'v' => Some(0.1),
        'e' => Some(0.05),
        'd' => Some(5.),
        's' => Some(10.),
        _ => None,
    }
}

fn temp_map(c: char) -> Option<u8> {
    match c {
        'n' => Some(100),
        'r' => Some(50),
        'o' => Some(15),
        'y' => Some(25),
        'l' => Some(10),
        'v' => Some(5),
        _ => None,
    }
}

fn convert(hundreds: u8, tens: u8, ones: u8, mult: f32, tolerance: f32) -> (f32, char, f32, char) {
    let mut suffix: char = ' ';
    let mut prefix: f32 = 0.;
    let mut tol_prefix: f32 = 0.;
    let mut tol_suffix: char = ' ';

    let whole = (hundreds as f32 * 100. + tens as f32 * 10. + ones as f32) * mult;
    if whole < 1000. {
        prefix = whole;
    } else if whole < 1000000. {
        prefix = whole / 1000.;
        suffix = 'K';
    } else if whole < 1000000000. {
        prefix = whole / 1000000.;
        suffix = 'M';
    } else if whole < 1000000000000. {
        prefix = whole / 1000000000.;
        suffix = 'G';
    }

    let applied_tol: f32 = tolerance / 100. * whole;

    if applied_tol < 1000. {
        tol_prefix = applied_tol;
    } else if applied_tol < 1000000. {
        tol_prefix = applied_tol / 1000.;
        tol_suffix = 'K';
    } else if applied_tol < 1000000000. {
        tol_prefix = applied_tol / 1000000.;
        tol_suffix = 'M';
    } else if applied_tol < 1000000000000. {
        tol_prefix = applied_tol / 1000000000.;
        tol_suffix = 'G';
    }

    (prefix, suffix, tol_prefix, tol_suffix)
}
