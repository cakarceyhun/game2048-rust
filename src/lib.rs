use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub enum DirectionEnum {
    Start,
    Left,
    Right,
    Up,
    Down
}

#[wasm_bindgen]
extern {
    pub fn alert(s: &str);
}

#[wasm_bindgen]
pub fn process(_direction: DirectionEnum) -> String {
    return "CEYHUN".to_string();
}

