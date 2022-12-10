use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Player {
    User,
    Computer,
}

pub type Claimed = Option<Player>;

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

    pub fn claim(&mut self, player: Player) {
        self.claimed = Some(player);
    }

    pub fn determine_claim(
        &mut self,
        vertical_edges: &Vec<Claimed>,
        horizontal_edges: &Vec<Claimed>,
    ) -> bool {
        let all_vertical_edges_claimed = self
            .vertical_edges
            .iter()
            .all(|index| vertical_edges[*index].is_some());

        let all_horizontal_edges_claimed = self
            .horizontal_edges
            .iter()
            .all(|index| horizontal_edges[*index].is_some());

        all_vertical_edges_claimed && all_horizontal_edges_claimed
    }
}
