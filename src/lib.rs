mod utils;

use wasm_bindgen::prelude::*;

use writing_tools::check;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet() {
    let input = String::from("Vivamus lacinia, libero ac lobortis iaculis, dui dui malesuada diam, eu interdum tellus risus nec leo class aptent taciti sociosqu ad litora torquent per conubia nostra, per inceptos himenaeos. Pellentesque vel elit maximus, suscipit turpis a, elementum turpis. Vivamus sollicitudin arcu sit amet elementum fermentum. Vestibulum sed velit in dolor molestie congue. Vestibulum dui quam, pharetra non egestas id, ullamcorper et mauris. Vestibulum blandit felis quis ligula finibus commodo.");
    let result = check::sentence_length::sentence_length(input);
    alert(&result);
}
