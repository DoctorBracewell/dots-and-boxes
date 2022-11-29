mod board;
mod properties;

use board::*;
use properties::*;
use wasm_bindgen::prelude::*;
use web_sys::console;

#[wasm_bindgen]
pub struct Game {
    pub width: usize,
    pub height: usize,
    pub difficulty: Difficulty,
    board: Vec<Vec<GameBox>>,
    vertical_edges: Vec<Claimed>,
    horizontal_edges: Vec<Claimed>,
}

#[wasm_bindgen]
impl Game {
    pub fn new(height: usize, width: usize) -> Self {
        let (board, vertical_edges, horizontal_edges) = Self::generate_board(height, width);

        Self {
            height,
            width,
            difficulty: Difficulty::Easy,
            board,
            vertical_edges,
            horizontal_edges,
        }
    }

    pub fn set_difficulty(&mut self, difficulty: Difficulty) {
        self.difficulty = difficulty;
    }

    fn generate_board(
        width: usize,
        height: usize,
    ) -> (Vec<Vec<GameBox>>, Vec<Claimed>, Vec<Claimed>) {
        let vertical_edges = vec![None; ((width + 1) * height) as usize];
        let horizontal_edges = vec![None; (width * (height + 1)) as usize];

        let mut board = vec![vec![GameBox::new(); width as usize]; height as usize];

        (board, vertical_edges, horizontal_edges)
    }

    pub fn print_board(&self) {
        for row in &self.board {
            for game_box in row {
                console::log_1(&format!("{:?}", game_box).into());
            }
        }
    }

    pub fn print_edges(&self) {
        for edge in &self.horizontal_edges {
            console::log_1(&format!("{:?}", edge).into());
        }

        for edge in &self.vertical_edges {
            console::log_1(&format!("{:?}", edge).into());
        }
    }

    pub fn claim_box(&mut self, x: usize, y: usize) {
        self.board[y][x].claim(Player::User);
    }

    // pub fn claim_edge(&mut self, x: usize, y: usize) {
    //     self.edges[y * (self.width as usize) + x] = Some(Player::User);
    // }
}
