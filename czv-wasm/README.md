# czv-wasm

WebAssembly (JavaScript and TypeScript) library for [czv](https://github.com/rzmk/czv). czv is a library of CSV-related operations for data engineering and analysis tasks.

## Links

-   czv GitHub repository: <https://github.com/rzmk/czv>
-   Rust: [crates.io/crates/czv](https://crates.io/crates/czv) ([source code](https://github.com/rzmk/czv/tree/main/czv))
-   WebAssembly (JavaScript/TypeScript): [npmjs.com/package/czv](https://www.npmjs.com/package/czv) ([source code](https://github.com/rzmk/czv/tree/main/czv-wasm))
-   Python: [pypi.org/project/czv](https://pypi.org/project/czv/) ([source code](https://github.com/rzmk/czv/tree/main/czv-python))

## Installation and example

```bash
bun install czv
```

Or use `npm`, `pnpm`, or `yarn` instead of `bun`.

```ts
import init, * as czv from "czv";
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

## Development

You must have [wasm-pack](https://rustwasm.github.io/wasm-pack/installer/) installed. If you have Cargo installed you may run:

```bash
cargo install wasm-pack
```

### Build WASM for web

```bash
wasm-pack build --release --target web --out-name czv
```

**Note**: Currently you must modify the `pkg/package.json` by replacing the name from `czv-wasm` to `czv`.

### Test WASM for browser

```bash
wasm-pack test --firefox --release
```

You may replace `--firefox` with another browser such as `--chrome` and `--safari`.
