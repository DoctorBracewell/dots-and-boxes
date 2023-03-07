use js_sys::Uint32Array;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[derive(Debug, Clone, Copy)]
pub enum Difficulty {
    Easy,
    Medium,
    Hard,
}

#[wasm_bindgen]
#[derive(Debug, Copy, Clone)]
pub enum LineType {
    Horizontal,
    Vertical,
}

#[wasm_bindgen]
pub struct TurnInformation(pub usize, pub LineType, Box<[u32]>);

#[wasm_bindgen]
impl TurnInformation {
    #[wasm_bindgen(constructor)]
    pub fn new(index: usize, line_type: LineType, affected_boxes: Box<[u32]>) -> Self {
        Self(index, line_type, affected_boxes)
    }

    #[wasm_bindgen(getter)]
    pub fn affected_boxes(&self) -> Box<[u32]> {
        self.2.clone()
    }
}

pub enum EdgeType {
    HorizontalNear,
    HorizontalFar,
    VerticalNear,
    VerticalFar,
    HorizontalShared,
    VerticalShared,
}

pub type GameBoxIndices = Vec<[usize; 2]>;
