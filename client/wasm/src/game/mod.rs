mod board;
mod properties;

use board::*;
use properties::{EdgeType::*, *};

use wasm_bindgen::prelude::*;
use web_sys::console;

use crate::utils::log;

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
        let game_box_indices = match self.edge_type(index, &line_type) {
            HorizontalNear => vec![(0, index % self.width)],
            HorizontalFar => vec![(4, index % self.width)],
            VerticalNear => vec![(index / self.width, 0)],
            VerticalFar => vec![((index / (self.width)) - 1, 4)],
            HorizontalShared => vec![
                (index / self.width, index % self.width),
                (index / self.width - 1, index % self.width),
            ],
            VerticalShared => vec![
                (index / (self.width + 1), index % (self.width + 1)),
                (index / (self.width + 1), index % (self.width + 1) - 1),
            ],
        };

        self.claim_boxes(game_box_indices);
    }

    fn claim_boxes(&mut self, game_box_indices: Vec<(usize, usize)>) {
        let mut any_box_claimed = false;

        for (y, x) in game_box_indices.iter() {
            let game_box = &mut self.board[*y][*x];

            if game_box.claimed.is_some() {
                continue;
            }

            let should_claim =
                game_box.determine_claim(&self.vertical_edges, &self.horizontal_edges);

            // log(&should_claim);

            if should_claim {
                game_box.claim(self.current_player);
                any_box_claimed = true;
            }
        }

        if !any_box_claimed {
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

    fn edge_type(&self, index: usize, line_type: &LineType) -> EdgeType {
        match line_type {
            LineType::Horizontal => {
                if index < self.width {
                    HorizontalNear
                } else if index >= self.width * self.height {
                    HorizontalFar
                } else {
                    HorizontalShared
                }
            }
            LineType::Vertical => {
                if index % (self.width + 1) == 0 {
                    VerticalNear
                } else if index % (self.width + 1) == self.width {
                    VerticalFar
                } else {
                    VerticalShared
                }
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
