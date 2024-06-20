"""
# czv

Python library for [czv](https://github.com/rzmk/czv). CSV content manipulation and analysis.

## Install

```bash
pip install czv
```

## Example

```python
from czv import row_count

data = \"""fruits,price
apple,2.50
banana,3.00
strawberry,1.50\"""

output = row_count(data, False)

print(output)
```

"""

from typing import Optional

def row_count(file_data: str, include_header_row: Optional[bool]) -> int:
    """Returns a count of the total number of rows.
    
    ## Arguments
     
    * `file_data` - CSV file data.
    * `include_header_row` - Specify whether to include the header row (first row) in the row count. Default is false.
    """

def column_count(file_data: str) -> int:
    """Returns a count of the total number of columns (fields).
    
    ## Arguments
    
    * `file_data` - CSV file data.
    """
