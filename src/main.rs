use std::fmt;
use std::io;

//FIXME panicks and crashes when inputs are bad instead of returning
fn main() {
    println!(
        "blac[K] brow[N] [R]ed [O]range [Y]ellow [G]reen b[L]ue [V]iolet gr[E]y [W]hite gol[D] [S]ilver"
    );

    loop {
        println!("Input color code: ");

        let mut input = String::new();

        io::stdin()
            .read_line(&mut input)
            .expect("failed to read line");

        let v: Vec<char> = input.trim().to_ascii_lowercase().chars().collect();

        let d: Bands = banding(v).expect("bad input"); //data

        let r = Resistor::new(d);

        println!("{}", r);
    }
}

#[derive(Clone, Debug)]
struct Resistor {
    prefix: f32,
    suffix: char,
    tolerance_prefix: f32,
    tolerance_suffix: char,
    temp: Option<u8>,
}

#[derive(Clone, Copy, Debug)]
struct Bands {
    hundreds: u8,
    tens: u8,
    ones: u8,
    mult: f32,
    tolerance: f32,
    temp: Option<u8>,
}

impl Resistor {
    fn new(b: Bands) -> Self {
        let (prefix, suffix, tolerance_prefix, tolerance_suffix) =
            notorize(b.hundreds, b.tens, b.ones, b.mult, b.tolerance);
        Resistor {
            prefix,
            suffix,
            tolerance_prefix,
            tolerance_suffix,
            temp: b.temp,
        }
    }
}

//TODO handling the white space could be more elegant
//but I think making them strings would consume more memory just to have ""
impl fmt::Display for Resistor {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.suffix != ' ' {
            write!(f, "{}{}Ω", self.prefix, self.suffix)?;
        } else {
            write!(f, "{}Ω", self.prefix)?;
        }

        if self.tolerance_prefix != 0.0 || self.tolerance_suffix != ' ' {
            if self.tolerance_suffix != ' ' {
                write!(f, " ± {}{}Ω", self.tolerance_prefix, self.tolerance_suffix)?;
            } else {
                write!(f, " ± {}Ω", self.tolerance_prefix)?;
            }
        }

        if let Some(ppm) = self.temp {
            write!(f, ", {}ppm/C", ppm)?;
        }

        Ok(())
    }
}

//TODO should I replace the 0's with None and make them options?
//like i did with temp?
fn banding(v: Vec<char>) -> Result<Bands, Box<dyn std::error::Error>> {
    match v.len() {
        3 => Ok(Bands {
            hundreds: 0,
            tens: prefix_map(v[0]).ok_or("char1")?,
            ones: prefix_map(v[1]).ok_or("char2")?,
            mult: suffix_map(v[2]).ok_or("char3")?,
            tolerance: 0.,
            temp: None,
        }),
        4 => Ok(Bands {
            hundreds: 0,
            tens: prefix_map(v[0]).ok_or("char1")?,
            ones: prefix_map(v[1]).ok_or("char2")?,
            mult: suffix_map(v[2]).ok_or("char3")?,
            tolerance: tolerance_map(v[3]).ok_or("char4")?,
            temp: None,
        }),
        5 => Ok(Bands {
            hundreds: prefix_map(v[0]).ok_or("char1")?,
            tens: prefix_map(v[1]).ok_or("char2")?,
            ones: prefix_map(v[2]).ok_or("char3")?,
            mult: suffix_map(v[3]).ok_or("char4")?,
            tolerance: tolerance_map(v[4]).ok_or("char5")?,
            temp: None,
        }),
        6 => Ok(Bands {
            hundreds: prefix_map(v[0]).ok_or("char1")?,
            tens: prefix_map(v[1]).ok_or("char2")?,
            ones: prefix_map(v[2]).ok_or("char3")?,
            mult: suffix_map(v[3]).ok_or("char4")?,
            tolerance: tolerance_map(v[4]).ok_or("char5")?,
            temp: temp_map(v[5]),
        }),
        _ => Err("unsupported length".into()),
    }
}

//FIXME these match functions happen at run time
//i want to explore compile time options
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

fn notorize(hundreds: u8, tens: u8, ones: u8, mult: f32, tolerance: f32) -> (f32, char, f32, char) {
    let whole = (hundreds as f32 * 100. + tens as f32 * 10. + ones as f32) * mult;
    let applied_tol: f32 = tolerance / 100. * whole;

    let (prefix, suffix) = fix_helper(whole);
    let (tol_prefix, tol_suffix) = fix_helper(applied_tol);
    (prefix, suffix, tol_prefix, tol_suffix)
}

fn fix_helper(number: f32) -> (f32, char) {
    match number {
        n if n < 1_000.0 => (n, ' '),
        n if n < 1_000_000.0 => (n / 1_000.0, 'K'),
        n if n < 1_000_000_000.0 => (n / 1_000_000.0, 'M'),
        n if n < 1_000_000_000_000.0 => (n / 1_000_000_000.0, 'G'),
        _ => (number, '?'),
    }
}
