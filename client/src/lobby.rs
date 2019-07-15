use crate::data::Map;
use valala_engine::prelude::{Action, Context, GameState, Scene};

pub struct Lobby;

impl GameState for Lobby {
    fn enter(&mut self, ctx: &Context, scene: &mut Scene) {
        let map = Map::new_hexagonal(scene);
        scene.add(Box::new(map));
    }

    fn frame(&mut self, ctx: &Context, scene: &mut Scene) -> Action {
        Action::Continue
    }
}
