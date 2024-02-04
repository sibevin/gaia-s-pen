use bevy::asset::AssetMetaCheck;
use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;
use bevy_tweening::*;
use bevy_ui_navigation::prelude::*;

use gaia_s_pen::{app, book};

fn main() {
    App::new()
        .insert_resource(ClearColor(app::theme::BG_COLOR))
        .insert_resource(AssetMetaCheck::Never)
        .add_systems(Startup, app::init::startup)
        .add_plugins((
            app::init::InitPlugin,
            ShapePlugin,
            DefaultNavigationPlugins,
            TweeningPlugin,
            book::BookPlugin,
        ))
        .run();
}
