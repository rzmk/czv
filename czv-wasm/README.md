# czv-wasm

WebAssembly (JavaScript and TypeScript) library for [czv](https://github.com/rzmk/czv). czv is a library of CSV-related operations for data engineering and analysis tasks.

-   For a Rust library see [czv](https://github.com/rzmk/czv/tree/main/czv).
-   For a Python library see [czv-python](https://github.com/rzmk/czv/tree/main/czv-python).

## Installation and example

```bash
bun install czv
```

Or use `npm`, `pnpm`, or `yarn` instead of `bun`.

```js
import init, * as czv from "czv";
// Must run `await init()` or `initSync()` first for web use
await init();

const data = `fruits,price
apple,2.50
banana,3.00
strawberry,1.50`;

const output = czv.rowCount(data);
console.log(output);
```

## Development

You must have [wasm-pack](https://rustwasm.github.io/wasm-pack/installer/) installed. If you have Cargo installed you may run:

```bash
cargo install wasm-pack
```

### Build WASM for web

```bash
wasm-pack build --release --target web
```

### Test WASM for browser

```bash
wasm-pack test --firefox --release
```

You may replace `--firefox` with another browser such as `--chrome` and `--safari`.
