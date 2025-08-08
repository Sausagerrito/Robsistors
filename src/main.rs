use std::fmt;
use std::io::{self, Write};

fn main() {
    println!(
        "blac[K] brow[N] [R]ed [O]range [Y]ellow [G]reen b[L]ue [V]iolet gr[E]y [W]hite gol[D] [S]ilver"
    );
    println!("Enter color codes separated by spaces. Type ':q' to exit.");

    loop {
        print!("Input color code: ");
        io::stdout().flush().expect("Failed to flush stdout");

        let mut input = String::new();

        io::stdin()
            .read_line(&mut input)
            .expect("failed to read line");

        let trimmed = input.trim();

        // Check for exit commands
        if trimmed.eq_ignore_ascii_case(":q") {
            break;
        }

        for code in trimmed.split_whitespace() {
            let v: Vec<char> = code.to_ascii_lowercase().chars().collect();

            match chars_to_bands(v) {
                Ok(d) => {
                    let r = Resistor::new(&d);
                    println!("{}", r);
                }
                Err(e) => {
                    println!("ERROR: bad input '{}': {}", code, e);
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

#[derive(Clone, Debug)]
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
    /// Creates a new Resistor from Bands
    fn new(b: &Bands) -> Self {
        let (value, tolerance) = bands_to_ohms(b);
        Resistor {
            value,
            tolerance,
            temp: b.temp,
        }
    }
}

impl fmt::Display for Resistor {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt_helper(f, eng_fmt(self.value), "")?;
        if self.tolerance != 0. {
            let (tol_val, tol_char) = eng_fmt(self.tolerance);
            fmt_helper(f, (tol_val, tol_char), " ± ")?;
        }
        if self.temp != 0 {
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
            tolerance: 0.,
            temp: 0,
        }),
        4 => Ok(Bands {
            hundreds: 0,
            tens: digit_map(v[0]).ok_or(BandError::BadDigit(1))?,
            ones: digit_map(v[1]).ok_or(BandError::BadDigit(2))?,
            mult: mult_map(v[2]).ok_or(BandError::BadMultiplier)?,
            tolerance: tolerance_map(v[3]).map_err(|_| BandError::BadTolerance)?,
            temp: 0,
        }),
        5 => Ok(Bands {
            hundreds: digit_map(v[0]).ok_or(BandError::BadDigit(1))?,
            tens: digit_map(v[1]).ok_or(BandError::BadDigit(2))?,
            ones: digit_map(v[2]).ok_or(BandError::BadDigit(3))?,
            mult: mult_map(v[3]).ok_or(BandError::BadMultiplier)?,
            tolerance: tolerance_map(v[4]).map_err(|_| BandError::BadTolerance)?,
            temp: 0,
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
        'k' => Some(0), // Black
        'n' => Some(1), // Brown
        'r' => Some(2), // Red
        'o' => Some(3), // Orange
        'y' => Some(4), // Yellow
        'g' => Some(5), // Green
        'l' => Some(6), // Blue
        'v' => Some(7), // Violet
        'e' => Some(8), // Grey
        'w' => Some(9), // White
        _ => None,
    }
}

fn mult_map(c: char) -> Option<f32> {
    match c {
        'k' => Some(1.),             // Black
        'n' => Some(10.),            // Brown
        'r' => Some(100.),           // Red
        'o' => Some(1_000.),         // Orange
        'y' => Some(10_000.),        // Yellow
        'g' => Some(100_000.),       // Green
        'l' => Some(1_000_000.),     // Blue
        'v' => Some(10_000_000.),    // Violet
        'e' => Some(100_000_000.),   // Grey
        'w' => Some(1_000_000_000.), // White
        'd' => Some(0.1),            // Gold
        's' => Some(0.01),           // Silver
        _ => None,
    }
}

fn tolerance_map(c: char) -> Result<f32, BandError> {
    match c {
        'n' => Ok(0.01),    // Brown
        'r' => Ok(0.02),    // Red
        'o' => Ok(0.03),    // Orange
        'y' => Ok(0.04),    // Yellow
        'g' => Ok(0.005),   // Green
        'l' => Ok(0.002_5), // Blue
        'v' => Ok(0.001),   // Violet
        'e' => Ok(0.000_5), // Grey
        'd' => Ok(0.05),    // Gold
        's' => Ok(0.1),     // Silver
        _ => Err(BandError::BadTolerance),
    }
}

fn temp_map(c: char) -> Result<u8, BandError> {
    match c {
        'k' => Ok(250), // Black
        'n' => Ok(100), // Brown
        'r' => Ok(50),  // Red
        'o' => Ok(15),  // Orange
        'y' => Ok(25),  // Yellow
        'g' => Ok(20),  // Green
        'l' => Ok(10),  // Blue
        'v' => Ok(5),   // Violet
        'e' => Ok(1),   // Grey
        _ => Err(BandError::BadTemperature),
    }
}

fn bands_to_ohms(raw: &Bands) -> (f32, f32) {
    let value: f32 =
        (raw.hundreds as f32 * 100. + raw.tens as f32 * 10. + raw.ones as f32) * raw.mult;
    let tolerance = raw.tolerance * value;
    (value, tolerance)
}

fn eng_fmt(number: f32) -> (f32, char) {
    match number {
        n if n == 0. => (n, ' '),
        n if n < 0.00_001 => (n, 'n'),
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
        assert_eq!(mult_map('w'), Some(1_000_000_000.));
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
        assert!(matches!(
            chars_to_bands(vec!['r']),
            Err(BandError::UnsupportedLength)
        ));
        assert!(matches!(
            chars_to_bands(vec!['r', 'r']),
            Err(BandError::UnsupportedLength)
        ));
        assert!(matches!(
            chars_to_bands(vec!['r', 'r', 'r', 'r', 'r', 'r', 'r']),
            Err(BandError::UnsupportedLength)
        ));
    }

    #[test]
    fn test_chars_to_bands_invalid_digit() {
        assert!(matches!(
            chars_to_bands(vec!['x', 'r', 'r']),
            Err(BandError::BadDigit(1))
        ));
        assert!(matches!(
            chars_to_bands(vec!['r', 'x', 'r']),
            Err(BandError::BadDigit(2))
        ));
        assert!(matches!(
            chars_to_bands(vec!['r', 'r', 'x']),
            Err(BandError::BadMultiplier)
        ));
    }

    #[test]
    fn test_chars_to_bands_invalid_tolerance() {
        assert!(matches!(
            chars_to_bands(vec!['r', 'r', 'r', 'z']),
            Err(BandError::BadTolerance)
        ));
    }

    #[test]
    fn test_chars_to_bands_invalid_temp() {
        assert!(matches!(
            chars_to_bands(vec!['r', 'r', 'r', 'k', 'n', 'z']),
            Err(BandError::BadTemperature)
        ));
    }

    #[test]
    fn test_bands_to_ohms() {
        let bands = Bands {
            hundreds: 1,
            tens: 2,
            ones: 3,
            mult: 10.,
            tolerance: 0.05,
            temp: 0,
        };
        let (value, tolerance) = bands_to_ohms(&bands);
        assert_eq!(value, 1230.);
        assert!((tolerance - 61.5).abs() < 1e-6);
    }

    #[test]
    fn test_eng_fmt() {
        assert_eq!(eng_fmt(0.), (0., ' '));
        assert_eq!(eng_fmt(0.000_000_5), (0.000_000_5, 'n'));
        assert_eq!(eng_fmt(0.000_5), (0.000_5, 'μ'));
        assert_eq!(eng_fmt(0.5), (0.5, 'm'));
        assert_eq!(eng_fmt(500.), (500., ' '));
        assert_eq!(eng_fmt(5000.), (5., 'K'));
        assert_eq!(eng_fmt(5_000_000.), (5., 'M'));
        assert_eq!(eng_fmt(5_000_000_000.), (5., 'G'));
    }
}
