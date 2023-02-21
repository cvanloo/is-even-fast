extern crate wasm_bindgen;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn is_even_fast(num: i32) -> bool {
    num % 2 == 0
}

#[cfg(test)]
mod tests {
    use crate::is_even_fast;
    #[test]
    fn it_is_even() {
        assert_eq!(is_even_fast(6), true);
        assert_eq!(is_even_fast(95312), true);
    }
    #[test]
    fn it_is_odd() {
        assert_eq!(is_even_fast(7), false);
        assert_eq!(is_even_fast(11253), false);
    }
}
