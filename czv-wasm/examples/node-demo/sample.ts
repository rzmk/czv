// See the README.md file in this file's folder
// for how to run this example
import czv from "../../pkg/czv-ts";

const options: czv.RowCountOptions = {
    file_data: `fruit,price,
apple,2.50
banana,3.00
strawberry,1.50`,
    include_header_row: true
};

const output: number = czv.rowCount(options);

console.log(output); // 4
