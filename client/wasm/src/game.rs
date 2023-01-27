mod board;
mod properties;

use board::*;
use properties::{EdgeType::*, *};

use js_sys::Uint32Array;
use wasm_bindgen::prelude::*;

use super::utils::log;

#[wasm_bindgen]
pub struct Game {
    width: usize,
    height: usize,
    difficulty: Difficulty,
    current_player: Player,
    board: Vec<Vec<GameBox>>,
    vertical_edges: Vec<Claimed>,
    horizontal_edges: Vec<Claimed>,
}

// Main Game logic
#[wasm_bindgen]
impl Game {
    #[wasm_bindgen(constructor)]
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

    pub fn interact_edge(&mut self, index: usize, line_type: LineType) -> Uint32Array {
        let edge = self.get_edge(index, line_type);

        let game_boxes = if let None = edge {
            self.claim_edge(index, &line_type);

            let game_box_indices = self.affected_boxes(index, &line_type);
            let any_box_claimed = self.claim_boxes(&game_box_indices);

            if !any_box_claimed {
                self.switch_player();
            }

            game_box_indices
                .into_iter()
                .flatten()
                .map(|index| index as u32)
                .collect::<Vec<u32>>()
        } else {
            vec![]
        };

        Uint32Array::from(&game_boxes[..])
    }

    fn claim_edge(&mut self, index: usize, line_type: &LineType) {
        // Claim the edge
        match line_type {
            LineType::Horizontal => self.horizontal_edges[index] = Some(self.current_player),
            LineType::Vertical => self.vertical_edges[index] = Some(self.current_player),
        }
    }

    fn affected_boxes(&self, index: usize, line_type: &LineType) -> GameBoxIndices {
        // Create a vector of the boxes to check (the ones that have been affected by the edge)
        let game_box_indices = match self.edge_type(index, line_type) {
            HorizontalNear => vec![[0, index % self.width]],
            HorizontalFar => vec![[self.height - 1, index % self.width]],
            VerticalNear => vec![[index / self.width, 0]],
            VerticalFar => vec![[(index / (self.width)) - 1, self.width - 1]],
            HorizontalShared => vec![
                [index / self.width, index % self.width],
                [index / self.width - 1, index % self.width],
            ],
            VerticalShared => vec![
                [index / (self.width + 1), index % (self.width + 1)],
                [index / (self.width + 1), index % (self.width + 1) - 1],
            ],
        };

        game_box_indices
    }

    fn claim_boxes(&mut self, game_box_indices: &GameBoxIndices) -> bool {
        let mut any_box_claimed = false;

        for [y, x] in game_box_indices.iter() {
            let game_box = &mut self.board[*y][*x];

            if game_box.claimed.is_some() {
                continue;
            }

            let should_claim =
                game_box.determine_claim(&self.vertical_edges, &self.horizontal_edges);

            if should_claim {
                game_box.claim(self.current_player);
                any_box_claimed = true;
            }
        }

        any_box_claimed
    }

    pub fn take_turn(&mut self) {
        self.set_current_player(Player::Computer);

        self.determine_optimal_move();

        self.set_current_player(Player::User);
    }
}

// Optimal-Play algorithm
#[wasm_bindgen]
impl Game {
    fn determine_optimal_move(&self) {
        self.is_looney();
    }

    fn is_looney(&self) {
        for game_box in self.board.iter().flatten() {
            let edges = game_box.edge_count(&self.vertical_edges, &self.horizontal_edges);

            if edges == 3 {
                self.start_chain(game_box);
            }
        }
    }

    fn start_chain(&self, starting_box: &GameBox) {
        let mut chain = vec![];

        // Follow boxes directly connected to the starting box and add them to the chain if they have two open edges
        for edge in starting_box.edges.iter() {
            let game_box = 

            if game_box.edge_count(&self.vertical_edges, &self.horizontal_edges) == 2 {
                chain.push(game_box);
            }
        }
    }
}

// Setters & Getters
#[wasm_bindgen]
impl Game {
    #[wasm_bindgen(getter)]
    pub fn height(&self) -> usize {
        self.height
    }

    #[wasm_bindgen(getter)]
    pub fn width(&self) -> usize {
        self.width
    }

    #[wasm_bindgen(getter)]
    pub fn difficulty(&self) -> Difficulty {
        self.difficulty
    }

    #[wasm_bindgen(getter)]
    pub fn current_player(&self) -> Player {
        self.current_player
    }

    #[wasm_bindgen(setter)]
    pub fn set_height(&mut self, height: usize) {
        self.height = height;
    }

    #[wasm_bindgen(setter)]
    pub fn set_width(&mut self, width: usize) {
        self.width = width;
    }

    #[wasm_bindgen(setter)]
    pub fn set_difficulty(&mut self, difficulty: Difficulty) {
        self.difficulty = difficulty;
    }

    #[wasm_bindgen(setter)]
    pub fn set_current_player(&mut self, player: Player) {
        self.current_player = player;
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
}

// Output & Util
#[wasm_bindgen]
impl Game {
    pub fn switch_player(&mut self) {
        self.current_player = match self.current_player {
            Player::User => Player::Computer,
            Player::Computer => Player::User,
        }
    }

    pub fn count_boxes(&self, player: Player) -> usize {
        let mut count = 0;

        for row in &self.board {
            for game_box in row {
                let add = match game_box.claimed {
                    Some(claimed_player) => {
                        if claimed_player == player {
                            1
                        } else {
                            0
                        }
                    }
                    None => 0,
                };

                count += add;
            }
        }

        count
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

    #[cfg(debug_assertions)]
    pub fn print_board(&self) {
        for row in &self.board {
            for game_box in row {
                log(game_box);
            }
        }
    }

    #[cfg(debug_assertions)]
    pub fn print_edges(&self) {
        for edge in &self.horizontal_edges {
            log(edge);
        }

        for edge in &self.vertical_edges {
            log(edge);
        }
    }
}
