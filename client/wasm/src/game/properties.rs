use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[derive(Debug, Clone, Copy)]
pub enum Difficulty {
    Easy,
    Medium,
    Hard,
}

#[wasm_bindgen]
#[derive(Debug)]
pub enum LineType {
    Horizontal,
    Vertical,
}

pub enum EdgeType {
    HorizontalNear,
    HorizontalFar,
    VerticalNear,
    VerticalFar,
    HorizontalShared,
    VerticalShared,
}
