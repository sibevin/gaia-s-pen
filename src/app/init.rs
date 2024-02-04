use crate::app;
use bevy::{
    prelude::*,
    window::{Cursor, CursorIcon, PresentMode, WindowMode, WindowTheme},
};
use bevy_persistent::prelude::*;

pub struct InitPlugin;

impl Plugin for InitPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(app::status::AppStatus::default())
            .add_plugins((
                DefaultPlugins.set(WindowPlugin {
                    primary_window: Some(Window {
                        title: app::APP_NAME.into(),
                        resolution: (app::WINDOW_W, app::WINDOW_H).into(),
                        present_mode: PresentMode::AutoVsync,
                        fit_canvas_to_parent: true,
                        prevent_default_event_handling: false,
                        window_theme: Some(WindowTheme::Dark),
                        enabled_buttons: bevy::window::EnabledButtons {
                            maximize: false,
                            ..Default::default()
                        },
                        cursor: Cursor {
                            icon: CursorIcon::Crosshair,
                            ..default()
                        },
                        ..default()
                    }),
                    ..default()
                }),
                app::timer::TimerPlugin,
                app::settings::SettingsPlugin,
                app::leaderboard::LeaderboardPlugin,
                app::achievement::AchievementPlugin,
                app::audio::AudioSeAssetPlugin,
                app::key_binding::KeyBindingPlugin,
                app::interaction::InteractionPlugin,
                app::anime_effect::AnimeEffectPlugin,
            ));
    }
}

pub fn startup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut audio_se_asset: ResMut<app::audio::AudioSeAsset>,
    settings: Res<Persistent<app::settings::Settings>>,
    mut window_query: Query<&mut Window>,
) {
    // window
    let mut window = window_query.single_mut();
    if settings.is_enabled("fullscreen") {
        window.mode = WindowMode::Fullscreen
    } else {
        window.mode = WindowMode::Windowed
    }

    // audio
    app::audio::startup(&mut commands, &asset_server, &mut audio_se_asset, &settings);

    // camera
    commands.spawn(Camera2dBundle::default());
}
