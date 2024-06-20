# czv

Rust library for [czv](https://github.com/rzmk/czv). czv is a library of utility functions for CSV-related data engineering and analysis tasks.

## Usage

You must have [Rust](https://www.rust-lang.org/tools/install) and Cargo installed (Cargo may be additionally installed when you install Rust with `rustup`).

To install `czv`, run:

```bash
cargo install czv
```

Let's say we want to print the total number of rows in a 4x3 CSV file `fruits.csv` including the header row:

```rust
use czv::{count::RowCount, Result};

fn main() -> Result<()> {
    let data = r#"fruits,price
apple,2.50
banana,3.00
strawberry,1.50"#;
    let output = RowCount::new()
        .file_data(data)
        .include_header_row(true)
        .execute()?;
    println!("{output}"); // 4
    Ok(())
}
```

When ran, this should be the standard output printed in the terminal:

```console
4
```

## Tests

To run the tests, run:

```bash
cargo test
```

## Benchmarks

To run the benchmarks, run:

```bash
cargo bench
```

For benchmarks we use [criterion.rs](https://github.com/bheisler/criterion.rs).
