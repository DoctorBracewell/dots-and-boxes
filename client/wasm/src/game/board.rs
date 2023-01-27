use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Player {
    User,
    Computer,
}

pub type Claimed = Option<Player>;
pub type Edges = Vec<Claimed>;

#[wasm_bindgen]
#[derive(Debug)]
pub struct GameBox {
    pub claimed: Claimed,
    vertical_edges: [usize; 2],
    horizontal_edges: [usize; 2],
}

impl GameBox {
    pub fn new(horizontal_edges: [usize; 2], vertical_edges: [usize; 2]) -> Self {
        Self {
            claimed: None,
            vertical_edges,
            horizontal_edges,
        }
    }

    pub fn edge_count(&self, vertical_edges: &Edges, horizontal_edges: &Edges) -> usize {
        let vertical_edges_count = self
            .vertical_edges_iterator()
            .filter(|index| vertical_edges[**index].is_some())
            .count();

        let horizontal_edges_count = self
            .horizontal_edges_iterator()
            .filter(|index| horizontal_edges[**index].is_some())
            .count();

        vertical_edges_count + horizontal_edges_count
    }

    pub fn claim(&mut self, player: Player) {
        self.claimed = Some(player);
    }

    pub fn determine_claim(&self, vertical_edges: &Edges, horizontal_edges: &Edges) -> bool {
        let all_vertical_edges_claimed = self
            .vertical_edges_iterator()
            .all(|index| vertical_edges[*index].is_some());

        let all_horizontal_edges_claimed = self
            .horizontal_edges_iterator()
            .all(|index| horizontal_edges[*index].is_some());

        all_vertical_edges_claimed && all_horizontal_edges_claimed
    }

    fn vertical_edges_iterator(&self) -> impl Iterator<Item = &usize> {
        self.vertical_edges.iter()
    }

    fn horizontal_edges_iterator(&self) -> impl Iterator<Item = &usize> {
        self.horizontal_edges.iter()
    }
}
