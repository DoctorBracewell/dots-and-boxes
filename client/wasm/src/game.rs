mod board;
mod properties;

use std::ops::AddAssign;

use board::*;
use petgraph::graph::{Edge, EdgeIndex, Node, NodeIndex};
use properties::{EdgeType::*, LineType::*, *};

use js_sys::Uint32Array;
use wasm_bindgen::prelude::*;

use petgraph::dot::Dot;
use petgraph::stable_graph::{StableGraph, StableUnGraph};
use petgraph::Undirected;

use rand::thread_rng;
use rand::Rng;

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
    graph: StableUnGraph<GraphNode, ()>,
}

// Main Game logic
#[wasm_bindgen]
impl Game {
    #[wasm_bindgen(constructor)]
    pub fn new(height: usize, width: usize) -> Self {
        let vertical_edges = vec![None; (width + 1) * height];
        let horizontal_edges = vec![None; width * (height + 1)];

        let board = Game::generate_board(height, width);
        let mut graph: StableUnGraph<GraphNode, ()> = StableGraph::default();
        let mut boxes: Vec<Vec<NodeIndex>> = vec![];

        let ground = graph.add_node(GraphNode::Ground);

        for row in 0..height {
            boxes.push(vec![]);

            for _ in 0..width {
                let node_index = graph.add_node(GraphNode::Box);

                boxes[row].push(node_index);
            }
        }

        // Add horizontal edges
        for row in 0..height {
            for column in 0..width {
                let box_node = boxes[row][column];

                if row == 0 {
                    graph.add_edge(box_node, ground, ());
                } else {
                    graph.add_edge(box_node, boxes[row - 1][column], ());
                }
            }
        }

        for column in 0..width {
            graph.add_edge(boxes[height - 1][column], ground, ());
        }

        // Add vertical edges
        for row in 0..height {
            for column in 0..width {
                let box_node = boxes[row][column];

                if column == 0 {
                    graph.add_edge(box_node, ground, ());
                } else {
                    graph.add_edge(box_node, boxes[row][column - 1], ());
                }

                if column == width - 1 {
                    graph.add_edge(box_node, ground, ());
                }
            }
        }

        // Output graph in DOT format
        // log(Dot::new(&graph));
        // log(graph.edge_count());

        Self {
            height,
            width,
            difficulty: Difficulty::Easy,
            current_player: Player::User,
            board,
            vertical_edges,
            horizontal_edges,
            graph,
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

    pub fn interact_edge(&mut self, index: usize, line_type: LineType) -> Vec<u32> {
        let edge = self.get_edge(index, line_type);

        let game_boxes = if let None = edge {
            self.claim_edge(index, &line_type);

            let game_box_indices = self.affected_boxes(index, &line_type);
            let any_box_claimed = self.claim_boxes(&game_box_indices);

            // Remove edges from graph
            self.remove_edge(&game_box_indices);

            if !any_box_claimed || self.board_full() {
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

        game_boxes
    }

    pub fn handle_edge_interact(&mut self, index: usize, line_type: LineType) -> Uint32Array {
        Uint32Array::from(&self.interact_edge(index, line_type)[..])
    }

    fn remove_edge(&mut self, indices: &GameBoxIndices) {
        let a = (indices[0][0] * self.width + indices[0][1]) + 1;
        let b = match indices.get(1) {
            Some(index) => (index[0] * self.width + index[1]) + 1,
            None => 0,
        };

        let (node_one, node_two) = (NodeIndex::new(a), NodeIndex::new(b));

        let edge_to_remove = self.graph.find_edge(node_one, node_two).unwrap();

        self.graph.remove_edge(edge_to_remove);
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

    pub fn computer_turn(&mut self) -> TurnInformation {
        self.play_optimal_move()
    }
}

// Optimal-Play algorithm
#[wasm_bindgen]
impl Game {
    fn play_optimal_move(&mut self) -> TurnInformation {
        let chains = self.count_chains();

        // log(&chains);

        // determine if a particular edge is claimed
        let mut rng = thread_rng();

        let mut index = 0;
        let mut line_type = LineType::Horizontal;
        let mut is_claimed = self.get_edge(index, line_type).is_some();

        while is_claimed {
            line_type = match rng.gen_range(0..2) {
                0 => LineType::Horizontal,
                1 => LineType::Vertical,
                _ => LineType::Horizontal,
            };

            index = rng.gen_range(0..30);

            is_claimed = self.get_edge(index, line_type).is_some();
        }

        // if chains.len() >= 1 {
        //     self.make_looney_move(chains)
        // } else {
        // self.switch_player();
        TurnInformation::new(
            index,
            line_type,
            self.interact_edge(index, line_type).into(),
        )
        // }

        // let looney = self.is_looney(&chains);

        // if looney {
        //     log("is looney");
        //     self.make_looney_move(chains);
        // } else {
        //     self.make_optimal_move(&chains);
        // }
    }

    fn count_chains(&self) -> Vec<Vec<NodeIndex>> {
        // Initialise vectors to track chains and visited boxes
        let mut visited_nodes: Vec<NodeIndex> = vec![];
        let mut chains: Vec<Vec<NodeIndex>> = vec![];

        // Iterate over all boxes
        for node in self.graph.node_indices().skip(1) {
            // Do not start chains from boxes that have already been visited
            if !visited_nodes.contains(&node) {
                let chain = self.start_chain(node, &visited_nodes);

                // If a chain is found, add it to the list of chains and append all of its boxes to visited nodes
                if chain.len() != 0 {
                    visited_nodes.extend(chain.iter());
                    chains.push(chain);
                }
            }
        }

        chains
    }

    fn start_chain(
        &self,
        current_node: NodeIndex,
        visited_nodes: &Vec<NodeIndex>,
    ) -> Vec<NodeIndex> {
        let mut chain: Vec<NodeIndex> = vec![];
        let neighbours: Vec<NodeIndex> = self.graph.neighbors(current_node).collect();

        if neighbours.len() == 1 {
            chain.push(current_node);
            self.continue_chain(neighbours[0], &mut chain, visited_nodes);
        }

        return chain;
    }

    fn continue_chain(
        &self,
        current_node: NodeIndex,
        chain: &mut Vec<NodeIndex>,
        visited_nodes: &Vec<NodeIndex>,
    ) {
        let neighbors: Vec<NodeIndex> = self.graph.neighbors(current_node).collect();
        let length = neighbors.len();

        if let GraphNode::Ground = self.graph.node_weight(current_node).unwrap() {
            chain.push(current_node);
            return;
        }

        if length > 2 {
            chain.push(current_node);
            return;
        }

        if neighbors.iter().any(|node| visited_nodes.contains(&node)) {
            chain.clear();
            return;
        }

        chain.push(current_node);

        if length == 1 {
            return;
        }

        let next_node = neighbors
            .iter()
            .filter(|node| !chain.contains(node))
            .next()
            .unwrap();

        self.continue_chain(*next_node, chain, visited_nodes);
    }

    fn is_looney(&self, chains: &Vec<Vec<NodeIndex>>) -> bool {
        chains.iter().all(|chain| chain.len() >= 3)
            && chains
                .into_iter()
                .flatten()
                .collect::<Vec<&NodeIndex>>()
                .len()
                == self.width * self.height
    }

    fn make_looney_move(&mut self, chains: Vec<Vec<NodeIndex>>) -> TurnInformation {
        let shortest_chain = chains
            .iter()
            .reduce(|a, b| if a.len() > b.len() { a } else { b })
            .unwrap();

        let length = shortest_chain.len();

        // log(shortest_chain);

        let (node_1, node_2) = if chains.len() == 1 {
            (shortest_chain[0], shortest_chain[1])
        } else if length >= 4 {
            (shortest_chain[0], shortest_chain[1])
        } else {
            (shortest_chain[length - 1], shortest_chain[length - 2])
        };

        // log(node_1);
        // log(node_2);

        let (index, line_type) = self.graph_edge_to_board(node_1, node_2);

        TurnInformation::new(
            index,
            line_type,
            self.interact_edge(index, line_type).into(),
        )
    }

    fn graph_edge_to_board(&self, node_1: NodeIndex, node_2: NodeIndex) -> (usize, LineType) {
        let edge = self.graph.find_edge(node_1, node_2).unwrap().index();
        let horizontal_edge_count = self.width * (self.height + 1);

        let line_type = if edge <= horizontal_edge_count - 1 {
            LineType::Horizontal
        } else {
            LineType::Vertical
        };

        let line_index = match line_type {
            Horizontal => edge,
            Vertical => edge - horizontal_edge_count,
        };

        (line_index, line_type)
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

    pub fn board_full(&mut self) -> bool {
        self.board
            .iter()
            .all(|row| row.iter().all(|game_box| game_box.claimed.is_some()))
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
