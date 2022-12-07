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
    pub current_player: Player,
    board: Vec<Vec<GameBox>>,
    vertical_edges: Vec<Claimed>,
    horizontal_edges: Vec<Claimed>,
}

#[wasm_bindgen]
impl Game {
    pub fn new(height: usize, width: usize) -> Self {
        let vertical_edges = vec![None; (width + 1) * height];
        let horizontal_edges = vec![None; width * (height + 1)];

        let board = Game::generate_board(height, width);

        Self {
            height,
            width,
            difficulty: Difficulty::Easy,
            current_player: Player::User,
            board,
            vertical_edges,
            horizontal_edges,
        }
    }

    pub fn set_difficulty(&mut self, difficulty: Difficulty) {
        self.difficulty = difficulty;
    }

    fn generate_board(height: usize, width: usize) -> Vec<Vec<GameBox>> {
        let mut board: Vec<Vec<GameBox>> = vec![];

        for i in 0..height {
            board.insert(i, vec![]);

            for j in 0..width {
                // top, bottom
                let vertical_edges = [i * width + j, i * width + j + width];

                // left, right
                let horizontal_edges = [i * (width + 1) + j, i * (width + 1) + j + 1];

                board[i].insert(j, GameBox::new(vertical_edges, horizontal_edges));
            }
        }

        board
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

    pub fn claim_edge(&mut self, index: usize, line_type: LineType) {
        // Claim the edge
        match line_type {
            LineType::Horizontal => self.horizontal_edges[index] = Some(self.current_player),
            LineType::Vertical => self.vertical_edges[index] = Some(self.current_player),
        }

        // Create a vector of the boxes to check (the ones that have been affected by the edge)
        let mut game_boxes = if self.index_is_edge(index, &line_type) {
            match line_type {
                LineType::Horizontal => {
                    vec![&mut self.board[(index / (self.height)) - 1][index % self.width]]
                }
                LineType::Vertical => {
                    // Check if the box should be claimed
                    vec![]
                }
            }
        } else {
            vec![]
        };

        let mut box_claimed = false;

        for game_box in game_boxes.iter_mut() {
            box_claimed = game_box.determine_claim(
                self.current_player,
                &self.vertical_edges,
                &self.horizontal_edges,
            );
        }

        if !box_claimed {
            self.switch_player();
        }
    }

    pub fn get_edge(&self, index: usize, line_type: LineType) -> Claimed {
        match line_type {
            LineType::Horizontal => self.horizontal_edges[index],
            LineType::Vertical => self.vertical_edges[index],
        }
    }

    pub fn get_box(&self, x: usize, y: usize) -> Claimed {
        self.board[y][x].claimed
    }

    fn index_is_edge(&self, index: usize, line_type: &LineType) -> bool {
        match line_type {
            LineType::Horizontal => index < self.width || index >= self.width * self.height,
            LineType::Vertical => {
                index % (self.width + 1) == 0 || index % (self.width + 1) == self.width
            }
        }
    }

    fn switch_player(&mut self) {
        self.current_player = match self.current_player {
            Player::User => Player::Computer,
            Player::Computer => Player::User,
        }
    }
}
