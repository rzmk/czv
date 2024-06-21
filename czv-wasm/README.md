# czv-wasm

WebAssembly (JavaScript and TypeScript) library for [czv](https://github.com/rzmk/czv). czv is a library of CSV-related operations for data engineering and analysis tasks.

## Links

-   czv GitHub repository: <https://github.com/rzmk/czv>
-   Rust: [crates.io/crates/czv](https://crates.io/crates/czv) ([source code](https://github.com/rzmk/czv/tree/main/czv))
-   WebAssembly (JavaScript/TypeScript): [npmjs.com/package/czv-wasm](https://www.npmjs.com/package/czv-wasm) ([source code](https://github.com/rzmk/czv/tree/main/czv-wasm))
-   Python: [pypi.org/project/czv](https://pypi.org/project/czv/) ([source code](https://github.com/rzmk/czv/tree/main/czv-python))

## Installation and example

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

## Development

You must have [wasm-pack](https://rustwasm.github.io/wasm-pack/installer/) installed. If you have Cargo installed you may run:

```bash
cargo install wasm-pack
```

### Build WASM for web

```bash
wasm-pack build --release --target web
```

**Note**: Currently you must modify the `pkg/package.json` by replacing the name from `czv-wasm` to `czv`.

### Test WASM for browser

```bash
wasm-pack test --firefox --release
```

You may replace `--firefox` with another browser such as `--chrome` and `--safari`.

### Publish package to npmjs.com

Getting a `Package name too similar to existing package csv` when trying to publish the package as `czv` so we publish the package as `czv-wasm`.

0. Build the `pkg` folder with `wasm-pack build --release --target web`.
1. Run `cd pkg`.
 <!-- 1. Based on https://github.com/rustwasm/wasm-pack/issues/427#issuecomment-435966644, rename the `name` in `pkg/package.json` from `czv-wasm` to `czv`. -->
2. Run `npm pkg fix` as suggested.
3. Run `npm publish`.
