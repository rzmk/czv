# czv-python

Python library for [czv](https://github.com/rzmk/czv). czv is a library of utility functions for CSV-related data engineering and analysis tasks.

## Installation and example

```bash
pip install czv
```

```python
import czv

data = """fruits,price
apple,2.50
banana,3.00
strawberry,1.50"""

output = czv.row_count(data, False)

print(output)
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
