use wasm_bindgen::prelude::*;
use web_sys::console;

#[wasm_bindgen]
#[derive(Debug, Clone, Copy)]
pub enum Difficulty {
    Easy,
    Medium,
    Hard,
}

#[wasm_bindgen]
pub struct Game {
    pub width: i32,
    pub height: i32,
    pub difficulty: Difficulty,
}

#[wasm_bindgen]
impl Game {
    pub fn new() -> Self {
        Self {
            height: 5,
            width: 5,
            difficulty: Difficulty::Easy,
        }
    }

    pub fn set_difficulty(&mut self, difficulty: Difficulty) {
        self.difficulty = difficulty;
    }

    pub fn log_difficulty(&self) {
        console::log_1(&format!("{:?}", self.difficulty).into());
    }
}
