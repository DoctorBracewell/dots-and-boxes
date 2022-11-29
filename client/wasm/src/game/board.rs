use wasm_bindgen::prelude::*;

#[derive(Debug, Clone, Copy)]
pub enum Player {
    User,
    Computer,
}

pub type Claimed = Option<Player>;

#[wasm_bindgen]
#[derive(Debug, Clone, Copy)]
pub struct GameBox {
    claimed: Claimed,
    edges: [Claimed; 4],
}

impl GameBox {
    pub fn new() -> Self {
        Self {
            claimed: None,
            edges: [None, None, None, None],
        }
    }

    pub fn claim(&mut self, player: Player) {
        self.claimed = Some(player);
    }

    pub fn set_edge(&mut self, edge: usize, claimed: Claimed) {
        self.edges[edge] = claimed;
    }
}
