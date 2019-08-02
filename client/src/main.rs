#![cfg_attr(feature = "strict", deny(warnings))]

mod store;
mod stage;
mod view;

use crate::stage::Title;
use clap::App;
use std::boxed::Box;
use std::error::Error;
use std::result::Result;
use valala_engine::prelude::{initialize, Context, Engine, ResourcePack, Settings};

fn main() -> Result<(), Box<dyn Error>> {
    initialize();

    App::new("Valala")
        .version(env!("CARGO_PKG_VERSION"))
        .get_matches();

    let context = {
        let settings = Settings::from_file("settings.ron");
        let resource_pack = ResourcePack::default();
        let mut context = Context::new(settings, resource_pack);

        context.load_texture("stone", "stone.png");
        context.load_texture("grass", "grass.png");
        context.load_texture("dirt", "dirt.png");
        context.load_texture("water", "water.png");
        context.load_texture("character", "character.png");

        context.load_shader("map", "map.vert", "map.frag");

        context.load_model("character", "character.obj");

        context
    };

    let store = store::create();

    let mut engine = Engine::new(context, store)?;

    engine.run(Box::new(Title));

    Ok(())
}
