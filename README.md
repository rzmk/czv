# czv

czv includes libraries for Rust, Python, and WebAssembly (JavaScript and TypeScript) providing CSV-related operations for data engineering and analysis tasks.

## Links

-   czv GitHub repository: <https://github.com/rzmk/czv>
-   Rust: [crates.io/crates/czv](https://crates.io/crates/czv) ([source code](https://github.com/rzmk/czv/tree/main/czv))
-   WebAssembly (JavaScript/TypeScript): [npmjs.com/package/czv-wasm](https://www.npmjs.com/package/czv-wasm) ([source code](https://github.com/rzmk/czv/tree/main/czv-wasm))
-   Python: [pypi.org/project/czv](https://pypi.org/project/czv/) ([source code](https://github.com/rzmk/czv/tree/main/czv-python))

## Installation and examples

In the following examples we'll get the total number of rows in the CSV data including the header row.

### Rust

```bash
cargo install czv
```

```rust
use czv::{RowCount, Result};

fn main() -> Result<()> {
    let data: &str = "\
fruits,price
apple,2.50
banana,3.00
strawberry,1.50
";
    let output: usize = RowCount::new().file_data(data).execute()?;
    println!("{output}"); // 3
    Ok(())
}
```

### JavaScript/TypeScript (WebAssembly)

```bash
bun install czv-wasm
```

Or use `npm`, `pnpm`, or `yarn` instead of `bun`.

The following example is written in TypeScript:

```ts
import init, * as czv from "czv-wasm";
// Must run `await init()` or `initSync()` first for web use
await init();

const data = `fruits,price
apple,2.50
banana,3.00
strawberry,1.50`;

const output: number = czv.rowCount({
    file_data: data,
    include_header_row: true,
});

console.log(output);
```

### Python

```bash
pip install czv
```

Or use [`uv pip`](https://github.com/astral-sh/uv) instead of `pip`.

```python
import czv

data: str = """fruits,price
apple,2.50
banana,3.00
strawberry,1.50"""

output: int = czv.row_count(file_data=data, include_header_row=True)

print(output) # 4
```

## Available operations

| czv (Rust)                               | czv-wasm (JS/TS)                       | czv-python                                      | Summary                   |
| ---------------------------------------- | -------------------------------------- | ----------------------------------------------- | ------------------------- |
| [`count::RowCount`](czv/src/count.rs)    | [`rowCount`](czv-wasm/src/count.rs)    | [`count.row_count`](czv-python/src/count.rs)    | Get the number of rows    |
| [`count::ColumnCount`](czv/src/count.rs) | [`columnCount`](czv-wasm/src/count.rs) | [`count.column_count`](czv-python/src/count.rs) | Get the number of columns |

## Development

Each package has its own `README.md` with more info for that particular package.

You may generate docs with:

```bash
cargo doc --no-deps --workspace --open
```

## Notes

czv is inspired by the command-line tools [xsv](https://github.com/BurntSushi/xsv) and [qsv](https://github.com/jqnatividad/qsv), but czv is not intended to cover all of their commands.

Not all provided libraries may be in sync at a given time. See the [available operations table](#available-operations) for a common operation list between libraries (not all implementations for a given operation may be in sync either, for example they may not have the same builder/function arguments).

Here are a few expected features for each provided operation:

-   czv (Rust)
    -   Provide both a builder (recommended and common for conditional parameters) and a function
    -   Provide documentation (docstrings) in Markdown format
-   czv-wasm (Web, JavaScript and TypeScript)
    -   Use camelCase for exported functions
    -   Include common browser support to run in-browser
    -   Provide documentation (dosctrings) in TSDoc format
-   czv-python
    -   Provide documentation (docstrings) and type hints for IDEs when developers are using the Python library (sourced from [czv-python/czv.pyi](czv-python/czv.pyi)) in Markdown format
