import czv from "../../pkg/czv-ts";

const options: czv.RowCountOptions = {
    file_data: `fruit,price,
apple,2.50
banana,3.00
strawberry,1.50`,
};

const output = czv.rowCount(options);

console.log(output); // 4
