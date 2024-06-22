use czv_wasm;
use czv_wasm::count::RowCountOptions;
use czv_wasm::Result;
use wasm_bindgen_test::*;
wasm_bindgen_test::wasm_bindgen_test_configure!(run_in_browser);

#[test]
#[wasm_bindgen_test]
fn row_count_nonheader() -> Result<()> {
    let options = RowCountOptions {
        file_data: "fruit,price
apple,2.00
banana,1.50
strawberry,3.00"
            .to_string(),
        include_header_row: Some(false),
    };
    let result = czv_wasm::count::row_count(options)?;
    assert_eq!(result, 3);
    Ok(())
}

#[test]
#[wasm_bindgen_test]
fn row_count_header() -> Result<()> {
    let options = RowCountOptions {
        file_data: "fruit,price
apple,2.00
banana,1.50
strawberry,3.00"
            .to_string(),
        include_header_row: Some(true),
    };
    let result = czv_wasm::count::row_count(options)?;
    assert_eq!(result, 4);
    Ok(())
}
