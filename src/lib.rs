use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn reverse( text: &str ) -> String {
    text.split_whitespace().rev().collect::<Vec<&str>>().join( " " )
}

#[wasm_bindgen]
pub fn factorial( x: u64 ) -> u64 {
    match x {
        0 | 1 => 1,
        _ => x * factorial( x - 1 ),
    }
}
