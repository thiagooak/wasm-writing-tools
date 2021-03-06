mod utils;

use wasm_bindgen::prelude::*;

use writing_tools::check;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
pub fn check(input: String) -> String {
    let working_version: String = input.clone();
    let mut result: String = String::new();
    let mut index: usize = 0;

    let mut errors = check::long_words::long_words(input.clone());
    // let mut sl_errors = check::sentence_length::sentence_length(input.clone());

    // errors.append(&mut sl_errors);

    errors.sort_by(|a, b| a.index_start.cmp(&b.index_start));

    // @TODO what happens when two erros start on the same line - HashMap with key = position and values = Vec<Markers>
    for e in errors {
        let (_, unprocessed_input) = working_version.split_at(index);
        let (unprecessed_before_error, _) = unprocessed_input.split_at(e.index_start - index);

        result.push_str(unprecessed_before_error);
        result.push_str("<span class=\"orange\" title=\"");
        result.push_str(&e.suggestion.clone());
        result.push_str("\">");
        result.push_str(&e.original.clone());
        result.push_str("</span>");

        index = e.index_start + e.original.len();
    }

    let (_, unprocessed_input) = working_version.split_at(index);
    result.push_str(unprocessed_input);

    return result;
}
