# czv-python

Python library for [czv](https://github.com/rzmk/czv). czv is a library of CSV-related operations for data engineering and analysis tasks.

-   For a Rust library see [czv](https://github.com/rzmk/czv/tree/main/czv).
-   For a WebAssembly (JavaScript, TypeScript) library see [czv-wasm](https://github.com/rzmk/czv/tree/main/czv-wasm).

## Installation and example

To install `czv`, run:

```bash
pip install czv
```

Let's say we want to print the total number of rows in a 4x3 CSV file `fruits.csv` including the header row:

```python
import czv

output = czv.row_count(file_path="fruits.csv", include_header_row=True)

print(output) # 4
```

## Development

You'll need to have [maturin](https://github.com/PyO3/maturin/) and [uv](https://github.com/astral-sh/uv) installed. Set up a local virtual environment in the `czv-python` folder by running:

```bash
uv venv
```

Make sure to activate the virtual environment (instructions should be provided in your terminal after running the previous command).

Once you've activated the virtual environment, install dependencies by running:

```bash
uv pip install -r requirements.txt
```

### Build package in local environment

```bash
maturin develop --uv --release
```

### Run tests

```bash
pytest
```
