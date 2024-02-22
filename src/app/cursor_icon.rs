use crate::app::{self, cursor::*};
use bevy::prelude::*;
use bevy_mod_picking::prelude::*;
use bevy_ui_navigation::NavRequestSystem;

#[derive(Resource, Default, Debug)]
pub enum AppCursorIconKind {
    #[default]
    Normal,
    Pointer,
    Hidden,
}

pub struct AppCursorIconPlugin;

impl Plugin for AppCursorIconPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(AppCursorIconThrottleTimer(Timer::from_seconds(
            THROTTLE_SECS,
            TimerMode::Repeating,
        )))
        .add_systems(Update, show_cursor_icon.after(NavRequestSystem));
    }
}

const THROTTLE_SECS: f32 = 0.05;

#[derive(Resource)]
struct AppCursorIconThrottleTimer(pub Timer);

#[derive(Component)]
pub struct AppCursorIcon {
    pub kind: AppCursorIconKind,
}

const CURSOR_ICON_SIZE: f32 = 52.0;
static NORMAL_ICON_PATH: &str = "images/icons/leaf-fill_1.5x.png";
static POINTER_ICON_PATH: &str = "images/icons/hand-pointing-fill_1.5x.png";
const NORMAL_ICON_POS_BIAS: Vec2 = Vec2::new(0.0, -CURSOR_ICON_SIZE);
const POINTER_ICON_POS_BIAS: Vec2 = Vec2::new(-CURSOR_ICON_SIZE * 0.5, 0.0);
const CURSOR_Z_INDEX: i32 = 100;

pub fn init_cursor_icon(commands: &mut Commands, asset_server: &Res<AssetServer>) {
    let icon = asset_server.load(NORMAL_ICON_PATH);
    commands.spawn((
        ImageBundle {
            style: Style {
                position_type: PositionType::Absolute,
                width: Val::Px(CURSOR_ICON_SIZE),
                height: Val::Px(CURSOR_ICON_SIZE),
                ..default()
            },
            image: UiImage::new(icon),
            z_index: ZIndex::Global(CURSOR_Z_INDEX),
            ..default()
        },
        AppCursorIcon {
            kind: AppCursorIconKind::Normal,
        },
        Interaction::None,
        Pickable::IGNORE,
    ));
}

pub fn set_curosr_icon(cursor_icon_query: &mut Query<&mut AppCursorIcon>, kind: AppCursorIconKind) {
    if let Ok(mut cursor_icon) = cursor_icon_query.get_single_mut() {
        cursor_icon.kind = kind;
    }
}

pub fn reset_curosr_icon(cursor_icon_query: &mut Query<&mut AppCursorIcon>) {
    set_curosr_icon(cursor_icon_query, AppCursorIconKind::Normal);
}

pub fn hide_curosr_icon(cursor_icon_query: &mut Query<&mut AppCursorIcon>) {
    set_curosr_icon(cursor_icon_query, AppCursorIconKind::Hidden);
}

const HIDDEN_POS: Vec2 = Vec2::new(app::WINDOW_W * -3.0, app::WINDOW_H * -3.0);

fn show_cursor_icon(
    mut cursor_icon_query: Query<(&mut Style, &mut UiImage, &AppCursorIcon)>,
    cursor: Res<AppCursorData>,
    asset_server: Res<AssetServer>,
    mut throttle_timer: ResMut<AppCursorIconThrottleTimer>,
    time: Res<Time>,
) {
    if throttle_timer.0.tick(time.delta()).just_finished() {
        if let Ok((mut style, mut image, cursor_icon)) = cursor_icon_query.get_single_mut() {
            match cursor_icon.kind {
                AppCursorIconKind::Normal => {
                    image.texture = asset_server.load(NORMAL_ICON_PATH);
                    style.left = Val::Px(cursor.window_pos.x + NORMAL_ICON_POS_BIAS.x);
                    style.top = Val::Px(cursor.window_pos.y + NORMAL_ICON_POS_BIAS.y);
                }
                AppCursorIconKind::Pointer => {
                    image.texture = asset_server.load(POINTER_ICON_PATH);
                    style.left = Val::Px(cursor.window_pos.x + POINTER_ICON_POS_BIAS.x);
                    style.top = Val::Px(cursor.window_pos.y + POINTER_ICON_POS_BIAS.y);
                }
                AppCursorIconKind::Hidden => {
                    style.left = Val::Px(HIDDEN_POS.x);
                    style.top = Val::Px(HIDDEN_POS.y);
                }
            }
        }
    }
}
