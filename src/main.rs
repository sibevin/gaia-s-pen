use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;
use bevy_tweening::*;
use bevy_ui_navigation::prelude::*;

use gaia_s_pen::{app, book};

fn main() {
    App::new()
        .add_systems(Startup, app::startup)
        .add_plugins((
            app::AppPlugin,
            ShapePlugin,
            DefaultNavigationPlugins,
            TweeningPlugin,
            book::BookPlugin,
        ))
        .run();
}
