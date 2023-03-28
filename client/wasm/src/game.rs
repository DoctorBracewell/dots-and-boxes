mod board;
mod properties;

use board::*;
use petgraph::graph::{EdgeIndex, NodeIndex};
use properties::{EdgeType::*, LineType::*, *};

use js_sys::Uint32Array;
use wasm_bindgen::prelude::*;

use petgraph::stable_graph::{StableGraph, StableUnGraph};

use rand::seq::SliceRandom;
use rand::Rng;

#[cfg(debug_assertions)]
use super::utils::log;

type BoxCollection = Vec<Vec<NodeIndex>>;

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
        let mut boxes: BoxCollection = vec![];

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

        // If the edge is not claimed
        let game_boxes = if let None = edge {
            self.claim_edge(index, &line_type);

            let game_box_indices = self.affected_boxes(index, &line_type);
            let any_box_claimed = self.claim_boxes(&game_box_indices);

            // Remove edges from graph
            self.remove_edge(&game_box_indices, line_type);

            // Only switch the player if a box was claimed (or if the board is full)
            if !any_box_claimed || self.board_full() {
                self.switch_player();
            }

            // Transform the affected boxes into an array of indices
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

    fn remove_edge(&mut self, indices: &GameBoxIndices, line_type: LineType) {
        // Extract the correct node indicies from the affected boxes.
        let a = (indices[0][0] * self.width + indices[0][1]) + 1;
        let b = match indices.get(1) {
            Some(index) => (index[0] * self.width + index[1]) + 1,
            None => 0,
        };

        // Calculate the edge between the two graph nodes
        let (node_one, node_two) = (NodeIndex::new(a), NodeIndex::new(b));
        let mut edge_to_remove = self.graph.find_edge(node_one, node_two).unwrap();

        // Check if a or b are corner boxes
        edge_to_remove = if b == 0
            && matches!(line_type, LineType::Horizontal)
            && self.graph.edges_connecting(node_one, node_two).count() > 1
        {
            // Index manipulation for corner boxes
            if a == 1 {
                EdgeIndex::new(edge_to_remove.index() - self.width * (self.height + 1))
            } else if a == self.width {
                EdgeIndex::new(edge_to_remove.index() - self.width * (self.height + 1) - 1)
            } else if a == self.width * self.height {
                EdgeIndex::new(edge_to_remove.index() - self.width * (self.height + 1))
            } else if a == self.width * self.height - self.width + 1 {
                EdgeIndex::new(edge_to_remove.index() - self.width * (self.height + 1) + 1)
            } else {
                edge_to_remove
            }
        } else {
            edge_to_remove
        };

        // Remove the correct edge
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
        // Index manipulation to get the affected boxes, determined by the type of line
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
        self.determine_optimal_move()
    }
}

// Optimal-Play algorithm
impl Game {
    fn determine_optimal_move(&mut self) -> TurnInformation {
        let chains = self.count_chains();
        let tubes = self.count_tubes();

        // log(format!("chains: {:?}", &chains));
        // log(format!("tubes: {:?}", &tubes));

        if self.is_looney(&chains, &tubes) {
            // log("looney");
            self.make_looney_move(chains)
        } else if chains.len() > 0 {
            // log("chains");
            self.make_claiming_move(chains)
        } else if self.is_quasi_looney(&chains, &tubes) {
            // log("quasi looney");
            self.make_claiming_move(tubes)
        } else {
            // log("random");
            self.make_random_move()
        }
    }

    fn is_looney(&self, chains: &BoxCollection, tubes: &BoxCollection) -> bool {
        let filtered_chains = self.remove_duplicates(chains);
        let filtered_tubes = self.remove_duplicates(tubes);

        let box_count = filtered_chains
            .iter()
            .chain(filtered_tubes.iter())
            .flatten()
            .count();

        filtered_chains.len() == 1 && box_count == self.count_open_boxes()
    }

    fn is_quasi_looney(&self, chains: &BoxCollection, tubes: &BoxCollection) -> bool {
        let filtered_tubes = self.remove_duplicates(tubes);

        let box_count = filtered_tubes.iter().flatten().count();

        chains.len() == 0 && box_count == self.count_open_boxes()
    }

    fn make_looney_move(&mut self, chains: BoxCollection) -> TurnInformation {
        let sole_chain = &chains[0];
        let length = sole_chain.len();

        // End of looped chain, quadruple cross
        let (node_1, node_2) = if length == 4
            && !matches!(
                self.graph.node_weight(sole_chain[length - 1]).unwrap(),
                GraphNode::Ground,
            )
            && self.count_open_boxes() != 4
        {
            (sole_chain[1], sole_chain[2])
        }
        // End of singular chain, double cross
        else if length > 3 || self.count_open_boxes() == 2 {
            (sole_chain[0], sole_chain[1])
        }
        // Singular chain, claim box
        else {
            (sole_chain[length - 1], sole_chain[length - 2])
        };

        // Calculate index and line type of edge
        let (index, line_type) = self.graph_edge_to_board(node_1, node_2);

        // Return this information to the front end as well as update the board array
        TurnInformation::new(
            index,
            line_type,
            self.interact_edge(index, line_type).into(),
        )
    }

    fn make_claiming_move(&mut self, collections: BoxCollection) -> TurnInformation {
        // Calculate the first two boxes in the shortest collection (tube or chain)
        let shortest_collection = collections
            .iter()
            .reduce(|acc, cur| if acc.len() > cur.len() { cur } else { acc })
            .unwrap();

        let (node_1, node_2) = (shortest_collection[0], shortest_collection[1]);

        // Calculate index and line type of edge
        let (index, line_type) = self.graph_edge_to_board(node_1, node_2);

        // Return this information to the front end as well as update the board array
        TurnInformation::new(
            index,
            line_type,
            self.interact_edge(index, line_type).into(),
        )
    }

    fn make_random_move(&mut self) -> TurnInformation {
        let mut moves: Vec<(usize, LineType)> = vec![];

        // For horizontal and vertical lines
        for i in 0..2 {
            // Determine line type
            let line_type = match i {
                0 => LineType::Horizontal,
                _ => LineType::Vertical,
            };

            // Determine number of lines
            let line_count = match line_type {
                LineType::Horizontal => self.horizontal_edges.len(),
                LineType::Vertical => self.vertical_edges.len(),
            };

            // FOr each unclaimed line
            for index in 0..line_count {
                if self.get_edge(index, line_type).is_none() {
                    // Calculate the boxes affected by this line
                    let affected_boxes = self.affected_boxes(index, &line_type);
                    let affected_boxes_edges: Vec<usize> = affected_boxes
                        .iter()
                        .map(|[y, x]| {
                            self.board[*y][*x]
                                .edge_count(&self.vertical_edges, &self.horizontal_edges)
                        })
                        .collect();

                    // Only add to the vec if no boxes will be affected
                    if !affected_boxes_edges.iter().any(|&x| x == 2) {
                        moves.push((index, line_type));
                    }
                }
            }
        }

        // Generate a random thread
        let mut rng = rand::thread_rng();

        // Pick a random valid move
        if moves.len() != 0 {
            let (index, line_type) = *moves.choose(&mut rng).unwrap();

            return TurnInformation::new(
                index,
                line_type,
                self.interact_edge(index, line_type).into(),
            );
        }

        // Generate a random valid move from the existing edges
        let mut index = 0;
        let mut line_type = LineType::Horizontal;

        while self.get_edge(index, line_type).is_some() {
            index = rng.gen_range(0..(self.width * self.height));
            // get random value from the LineType enum
            line_type = match rng.gen_range(0..2) {
                0 => LineType::Horizontal,
                _ => LineType::Vertical,
            };
        }

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

// Chains
impl Game {
    fn count_chains(&self) -> BoxCollection {
        // Initialise vectors to track chains and visited boxes
        let mut visited_nodes: Vec<NodeIndex> = vec![];
        let mut chains: BoxCollection = vec![];

        // Iterate over all boxes
        for node in self.graph.node_indices().skip(1) {
            // Do not start chains from boxes that have already been visited
            if !visited_nodes.contains(&node) {
                let chain = self.start_chain(node, &visited_nodes);

                // If a chain is found, add it to the list of chains and append all of its boxes to visited nodes
                if chain.len() != 0 {
                    visited_nodes.extend(chain.iter().filter(|&b| {
                        !matches!(self.graph.node_weight(*b).unwrap(), GraphNode::Ground)
                    }));

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
        // Get the neighbours of the current box
        let neighbors: Vec<NodeIndex> = self.graph.neighbors(current_node).collect();
        let length = neighbors.len();

        // If it is the ground node, end the chain
        if let GraphNode::Ground = self.graph.node_weight(current_node).unwrap() {
            chain.push(current_node);
            return;
        }

        // If there are more than two neighbours, end the chain
        if length > 2 {
            chain.push(current_node);
            return;
        }

        // If the current box is a neighbour of a box that has already been visited, end and CLEAR the chain
        if neighbors.iter().any(|node| visited_nodes.contains(&node)) {
            chain.clear();
            return;
        }

        // Otherwise add the current box to the chain
        chain.push(current_node);

        if length == 1 {
            return;
        }

        // Continue with the neighbour that has not been visited yet
        let next_node = neighbors
            .iter()
            .filter(|node| !chain.contains(node))
            .next()
            .unwrap();

        self.continue_chain(*next_node, chain, visited_nodes);
    }
}

// Tubes
impl Game {
    fn count_tubes(&self) -> BoxCollection {
        // Initialise vectors to track tubes and visited boxes
        let mut visited_nodes: Vec<NodeIndex> = vec![];
        let mut tubes: BoxCollection = vec![];

        // Iterate over all nodes except the ground
        for node in self.graph.node_indices().skip(1) {
            // Do not start tubes from boxes that have already been visited
            if !visited_nodes.contains(&node) {
                let tube = self.start_tube(node, &visited_nodes);

                // If a tube is found, add it to the list of tubes and append all of its boxes to visited nodes
                if tube.len() != 0 {
                    visited_nodes.extend(tube.iter().filter(|&b| {
                        !matches!(self.graph.node_weight(*b).unwrap(), GraphNode::Ground)
                    }));

                    tubes.push(tube);
                }
            }
        }

        tubes
    }

    fn start_tube(
        &self,
        current_node: NodeIndex,
        visited_nodes: &Vec<NodeIndex>,
    ) -> Vec<NodeIndex> {
        let mut tube: Vec<NodeIndex> = vec![];
        let mut neighbours: Vec<NodeIndex> = self.graph.neighbors(current_node).collect();

        // Sort so ground nodes are last
        neighbours.sort_unstable();
        neighbours.reverse();

        let ground_neighbour = neighbours
            .iter()
            .by_ref()
            .find(|node| matches!(self.graph.node_weight(**node).unwrap(), GraphNode::Ground));

        // Continue the tube if the starting box has 2 neighbours
        if neighbours.len() == 2 {
            // If the starting box is connected to ground, add it to the tube
            if ground_neighbour.is_some() {
                tube.push(*ground_neighbour.unwrap());
            }

            // Otherwise continue the tube
            tube.push(current_node);

            self.continue_tube(neighbours[0], &mut tube, visited_nodes);
        }

        if tube.len() >= 2 && !tube.ends_with(&[tube[0]]) {
            tube.clear();
        }

        return tube;
    }

    fn continue_tube(
        &self,
        current_node: NodeIndex,
        tube: &mut Vec<NodeIndex>,
        visited_nodes: &Vec<NodeIndex>,
    ) {
        let neighbors: Vec<NodeIndex> = self.graph.neighbors(current_node).collect();
        let neighbors_length = neighbors.len();

        // If the current node is ground, add it to the tube and return
        if matches!(
            self.graph.node_weight(current_node).unwrap(),
            GraphNode::Ground
        ) && tube.len() > 2
        {
            tube.push(current_node);
            return;
        }

        if tube.len() >= 3 && neighbors.contains(&tube[0]) {
            tube.push(current_node);
            tube.push(tube[0]);
            return;
        }

        if neighbors_length > 2 {
            tube.push(current_node);
            return;
        }

        if neighbors.iter().any(|node| visited_nodes.contains(&node)) {
            tube.clear();
            return;
        }

        tube.push(current_node);

        if neighbors_length == 1 {
            return;
        }

        let next_node = neighbors
            .iter()
            .filter(|node| {
                !tube
                    .iter()
                    .filter(|n| **n != NodeIndex::new(0))
                    .collect::<Vec<&NodeIndex>>()
                    .contains(node)
            })
            .next()
            .unwrap();

        self.continue_tube(*next_node, tube, visited_nodes);
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

    pub fn count_open_boxes(&self) -> usize {
        let mut count = 0;

        for row in &self.board {
            for game_box in row {
                count += match game_box.claimed {
                    Some(_) => 0,
                    None => 1,
                }
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

    fn remove_duplicates<'a>(&'a self, structure: &'a BoxCollection) -> Vec<Vec<&'a NodeIndex>> {
        // Remove duplicate items from the structure vector
        let mut unique_structure: Vec<Vec<&NodeIndex>> = vec![];

        for chain in structure {
            let mut unique_chain: Vec<&NodeIndex> = vec![];

            for node in chain {
                if !matches!(self.graph.node_weight(*node).unwrap(), GraphNode::Ground)
                    && !unique_chain.contains(&node)
                {
                    unique_chain.push(node);
                }
            }

            unique_structure.push(unique_chain);
        }

        unique_structure
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
