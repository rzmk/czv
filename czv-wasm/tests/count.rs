use czv_wasm;
use czv_wasm::Result;
use wasm_bindgen_test::*;
wasm_bindgen_test::wasm_bindgen_test_configure!(run_in_browser);

#[test]
#[wasm_bindgen_test]
fn basic() -> Result<()> {
    let result = czv_wasm::count::row_count(
        "fruit,price
apple,2.00
banana,1.50
strawberry,3.00"
            .to_string(),
        Some(false),
    )?;
    assert_eq!(result, 3);
    Ok(())
}
