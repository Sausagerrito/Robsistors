use std::fmt;
use std::io;

fn main() {
    println!(
        "blac[K] brow[N] [R]ed [O]range [Y]ellow [G]reen b[L]ue [V]iolet gr[E]y [W]hite gol[D] [S]ilver"
    );

    loop {
        println!("\nInput color code: ");

        let mut input = String::new();

        io::stdin()
            .read_line(&mut input)
            .expect("failed to read line");

        for code in input.trim().split_whitespace() {
            let v: Vec<char> = code.to_ascii_lowercase().chars().collect();

            match chars_to_bands(v) {
                Ok(d) => {
                    let r = Resistor::new(&d);
                    println!("{}", r);
                }
                Err(e) => {
                    println!("ERROR: bad input: {}", e);
                    continue;
                }
            }
        }
    }
}

#[derive(Debug)]
struct Resistor {
    value: f32,
    tolerance: f32,
    temp: u8,
}

#[derive(Debug)]
struct Bands {
    hundreds: u8,
    tens: u8,
    ones: u8,
    mult: f32,
    tolerance: f32,
    temp: u8,
}

#[derive(Clone,Debug)]
enum BandError {
    BadDigit(usize),
    BadMultiplier,
    BadTolerance,
    BadTemperature,
    UnsupportedLength,
}

impl std::fmt::Display for BandError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            BandError::BadDigit(pos) => write!(f, "bad digit at position {}", pos),
            BandError::BadMultiplier => write!(f, "bad multiplier character"),
            BandError::BadTolerance => write!(f, "bad tolerance character"),
            BandError::BadTemperature => write!(f, "bad temperature character"),
            BandError::UnsupportedLength => write!(f, "unsupported band length"),
        }
    }
}

impl std::error::Error for BandError {}

impl Resistor {
    fn new(b: &Bands) -> Self {
        let (value, tolerance) = bands_to_ohms(b);
        Resistor {
            value: value,
            tolerance: tolerance,
            temp: b.temp,
        }
    }
}

impl fmt::Display for Resistor {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt_helper(f, egr_fmt(self.value), "")?;
        if self.tolerance != 0.{
            let (tol_val, tol_char) = egr_fmt(self.tolerance);
            fmt_helper(f, (tol_val, tol_char), " ± ")?;
        }
        if self.temp != 0{
            write!(f, ", {}°C", self.temp)?;
        }
    Ok(())
    }
}

fn fmt_helper(f: &mut fmt::Formatter, val: (f32, char), prefix: &str) -> fmt::Result {
    if val.1 != ' ' {
        write!(f, "{}{}{}Ω", prefix, val.0, val.1)
    } else {
        write!(f, "{}{}Ω", prefix, val.0)
    }
}

fn chars_to_bands(v: Vec<char>) -> Result<Bands, BandError> {
    match v.len() {
        3 => Ok(Bands {
            hundreds: 0,
            tens: digit_map(v[0]).ok_or(BandError::BadDigit(1))?,
            ones: digit_map(v[1]).ok_or(BandError::BadDigit(2))?,
            mult: mult_map(v[2]).ok_or(BandError::BadMultiplier)?,
            tolerance: Ok(0.)?,
            temp: Ok(0)?,
        }),
        4 => Ok(Bands {
            hundreds: 0,
            tens: digit_map(v[0]).ok_or(BandError::BadDigit(1))?,
            ones: digit_map(v[1]).ok_or(BandError::BadDigit(2))?,
            mult: mult_map(v[2]).ok_or(BandError::BadMultiplier)?,
            tolerance: tolerance_map(v[3]).map_err(|_| BandError::BadTolerance)?,
            temp: Ok(0)?,
        }),
        5 => Ok(Bands {
            hundreds: digit_map(v[0]).ok_or(BandError::BadDigit(1))?,
            tens: digit_map(v[1]).ok_or(BandError::BadDigit(2))?,
            ones: digit_map(v[2]).ok_or(BandError::BadDigit(3))?,
            mult: mult_map(v[3]).ok_or(BandError::BadMultiplier)?,
            tolerance: tolerance_map(v[4]).map_err(|_| BandError::BadTolerance)?,
            temp: Ok(0)?,
        }),
        6 => Ok(Bands {
            hundreds: digit_map(v[0]).ok_or(BandError::BadDigit(1))?,
            tens: digit_map(v[1]).ok_or(BandError::BadDigit(2))?,
            ones: digit_map(v[2]).ok_or(BandError::BadDigit(3))?,
            mult: mult_map(v[3]).ok_or(BandError::BadMultiplier)?,
            tolerance: tolerance_map(v[4]).map_err(|_| BandError::BadTolerance)?,
            temp: temp_map(v[5]).map_err(|_| BandError::BadTemperature)?,
        }),
        _ => Err(BandError::UnsupportedLength),
    }
}


fn digit_map(c: char) -> Option<u8> {
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

fn mult_map(c: char) -> Option<f32> {
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

fn tolerance_map(c: char) -> Result<f32, BandError> {
    match c {
        'n' => Ok(0.01),
        'r' => Ok(0.02),
        'o' => Ok(0.03),
        'y' => Ok(0.04),
        'g' => Ok(0.005),
        'l' => Ok(0.0025),
        'v' => Ok(0.001),
        'e' => Ok(0.0005),
        'd' => Ok(0.05),
        's' => Ok(0.1),
        _ => Err(BandError::BadTolerance),
    }
}

fn temp_map(c: char) -> Result<u8, BandError> {
    match c {
        'k' => Ok(250),
        'n' => Ok(100),
        'r' => Ok(50),
        'o' => Ok(15),
        'y' => Ok(25),
        'g' => Ok(20),
        'l' => Ok(10),
        'v' => Ok(5),
        'e' => Ok(1),
        _ => Err(BandError::BadTemperature),
    }
}

fn bands_to_ohms(raw: &Bands) -> (f32, f32) {
    let value: f32 =
        (raw.hundreds as f32 * 100. + raw.tens as f32 * 10. + raw.ones as f32) * raw.mult;
    let tolerance = raw.tolerance * value;
    (value, tolerance)
}

fn egr_fmt(number: f32) -> (f32, char) {
    match number {
        n if n == 0. => (n, ' '),
        n if n < 0.000_001 => (n, 'n'),
        n if n < 0.001 => (n, 'μ'),
        n if n < 1. => (n, 'm'),
        n if n < 1_000. => (n, ' '),
        n if n < 1_000_000. => (n / 1_000., 'K'),
        n if n < 1_000_000_000. => (n / 1_000_000., 'M'),
        n if n < 1_000_000_000_000. => (n / 1_000_000_000., 'G'),
        _ => (number, '?'),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_digit_map_valid() {
        assert_eq!(digit_map('k'), Some(0));
        assert_eq!(digit_map('n'), Some(1));
        assert_eq!(digit_map('w'), Some(9));
    }

    #[test]
    fn test_digit_map_invalid() {
        assert_eq!(digit_map('x'), None);
        assert_eq!(digit_map(' '), None);
    }

    #[test]
    fn test_mult_map_valid() {
        assert_eq!(mult_map('k'), Some(1.));
        assert_eq!(mult_map('d'), Some(0.1));
        assert_eq!(mult_map('w'), Some(1000000000.));
    }

    #[test]
    fn test_mult_map_invalid() {
        assert_eq!(mult_map('z'), None);
        assert_eq!(mult_map(' '), None);
    }

    #[test]
    fn test_tolerance_map_valid() {
        assert!(matches!(tolerance_map('n'), Ok(0.01)));
        assert!(matches!(tolerance_map('s'), Ok(0.1)));
    }

    #[test]
    fn test_tolerance_map_invalid() {
        assert!(matches!(tolerance_map('z'), Err(BandError::BadTolerance)));
        assert!(matches!(tolerance_map(' '), Err(BandError::BadTolerance)));
    }

    #[test]
    fn test_temp_map_valid() {
        assert!(matches!(temp_map('k'), Ok(250)));
        assert!(matches!(temp_map('e'), Ok(1)));
    }

    #[test]
    fn test_temp_map_invalid() {
        assert!(matches!(temp_map('z'), Err(BandError::BadTemperature)));
        assert!(matches!(temp_map(' '), Err(BandError::BadTemperature)));
    }

    #[test]
    fn test_chars_to_bands_valid_3band() {
        let bands = chars_to_bands(vec!['r', 'r', 'r']).unwrap();
        assert_eq!(bands.hundreds, 0);
        assert_eq!(bands.tens, 2);
        assert_eq!(bands.ones, 2);
        assert_eq!(bands.mult, 100.);
        assert_eq!(bands.tolerance, 0.);
        assert_eq!(bands.temp, 0);
    }

    #[test]
    fn test_chars_to_bands_valid_4band() {
        let bands = chars_to_bands(vec!['r', 'r', 'r', 'n']).unwrap();
        assert_eq!(bands.tolerance, 0.01);
    }

    #[test]
    fn test_chars_to_bands_valid_5band() {
        let bands = chars_to_bands(vec!['r', 'r', 'r', 'k', 'n']).unwrap();
        assert_eq!(bands.hundreds, 2);
        assert_eq!(bands.tens, 2);
        assert_eq!(bands.ones, 2);
        assert_eq!(bands.mult, 1.);
        assert_eq!(bands.tolerance, 0.01);
        assert_eq!(bands.temp, 0);
    }

    #[test]
    fn test_chars_to_bands_valid_6band() {
        let bands = chars_to_bands(vec!['r', 'r', 'r', 'k', 'n', 'k']).unwrap();
        assert_eq!(bands.temp, 250);
    }

    #[test]
    fn test_chars_to_bands_invalid_length() {
        assert!(matches!(chars_to_bands(vec!['r']), Err(BandError::UnsupportedLength)));
        assert!(matches!(chars_to_bands(vec!['r', 'r']), Err(BandError::UnsupportedLength)));
        assert!(matches!(chars_to_bands(vec!['r', 'r', 'r', 'r', 'r', 'r', 'r']), Err(BandError::UnsupportedLength)));
    }

    #[test]
    fn test_chars_to_bands_invalid_digit() {
        assert!(matches!(chars_to_bands(vec!['x', 'r', 'r']), Err(BandError::BadDigit(1))));
        assert!(matches!(chars_to_bands(vec!['r', 'x', 'r']), Err(BandError::BadDigit(2))));
        assert!(matches!(chars_to_bands(vec!['r', 'r', 'x']), Err(BandError::BadMultiplier)));
    }

    #[test]
    fn test_chars_to_bands_invalid_tolerance() {
        assert!(matches!(chars_to_bands(vec!['r', 'r', 'r', 'z']), Err(BandError::BadTolerance)));
    }

    #[test]
    fn test_chars_to_bands_invalid_temp() {
        assert!(matches!(chars_to_bands(vec!['r', 'r', 'r', 'k', 'n', 'z']), Err(BandError::BadTemperature)));
    }

    #[test]
    fn test_bands_to_ohms() {
        let bands = Bands {
            hundreds: 1,
            tens: 2,
            ones: 3,
            mult: 10.0,
            tolerance: 0.05,
            temp: 0,
        };
        let (value, tolerance) = bands_to_ohms(&bands);
        assert_eq!(value, 1230.0);
        assert!((tolerance - 61.5).abs() < 1e-6);
    }

    #[test]
    fn test_egr_fmt() {
        assert_eq!(egr_fmt(0.0), (0.0, ' '));
        assert_eq!(egr_fmt(0.000_000_5), (0.000_000_5, 'n'));
        assert_eq!(egr_fmt(0.000_5), (0.000_5, 'μ'));
        assert_eq!(egr_fmt(0.5), (0.5, 'm'));
        assert_eq!(egr_fmt(500.0), (500.0, ' '));
        assert_eq!(egr_fmt(5000.0), (5.0, 'K'));
        assert_eq!(egr_fmt(5_000_000.0), (5.0, 'M'));
        assert_eq!(egr_fmt(5_000_000_000.0), (5.0, 'G'));
    }
}