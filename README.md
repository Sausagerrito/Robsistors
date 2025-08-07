# Robsistors

A fast, minimal, and robust command-line resistor color code calculator written in Rust.

## Features
- Supports 3, 4, 5, and 6-band resistor color codes
- Handles all standard EIA color codes
- Displays resistance, tolerance, and temperature coefficient (if present)
- Clear error messages for invalid input
- SI prefix formatting for readable output (e.g., 4.7KΩ)
- Graceful exit with ':q'command
- Multiple resistor codes can be processed in one line
- Unit tested for reliability
- Tiny, optimized binary

## Usage

### Build

```
cargo build --release
```

The optimized binary will be in `target/release/robsistors.exe` (Windows) or `target/release/robsistors` (Unix).

### Run

```
./target/release/robsistors
```


You will see a prompt:

```
blac[K] brow[N] [R]ed [O]range [Y]ellow [G]reen b[L]ue [V]iolet gr[E]y [W]hite gol[D] [S]ilver
Enter color codes separated by spaces. Type ':q' to exit.
Input color code:
```

Type the color code letters as a single string (e.g., `rrrn` for red-red-red-brown, `rrrr` for red-red-red-red, `rrlr` for red-red-blue-red), then press Enter. Inputs can be strung together separated by whitespace (e.g. `rrrn rrrr rrlr`). Type `quit` or `exit` to exit the program.

#### Example

```
Input color code: rrr
2.2KΩ
Input color code: rrrr
2.2KΩ ± 44Ω
Input color code: rrrrr
22.2KΩ ± 444Ω
Input color code: quit
Goodbye!
```

- Invalid input will print a clear error message.

## Color Code Reference

| Color   | Letter | Digit | Multiplier | Tolerance | Temp Coeff |
|---------|--------|-------|------------|-----------|------------|
| Black   | k      | 0     | x1         |           | 250 ppm    |
| Brown   | n      | 1     | x10        | 1%        | 100 ppm    |
| Red     | r      | 2     | x100       | 2%        | 50 ppm     |
| Orange  | o      | 3     | x1k        | 3%        | 15 ppm     |
| Yellow  | y      | 4     | x10k       | 4%        | 25 ppm     |
| Green   | g      | 5     | x100k      | 0.5%      | 20 ppm     |
| Blue    | l      | 6     | x1M        | 0.25%     | 10 ppm     |
| Violet  | v      | 7     | x10M       | 0.1%      | 5 ppm      |
| Grey    | e      | 8     | x100M      | 0.05%     | 1 ppm      |
| White   | w      | 9     | x1G        |           |            |
| Gold    | d      |       | x0.1       | 5%        |            |
| Silver  | s      |       | x0.01      | 10%       |            |

## Testing

Run all unit tests with:

```
cargo test
```

## Optimizations
- Release builds use LTO, panic=abort, and symbol stripping for a tiny binary
- No runtime allocations for color code lookup

## License

MIT

---

This readme was AI generated, but the code was not. Critique is welcome and will help me improve :-)    
