use crate::app::ui::*;

pub mod slider;

const UI_CANVAS_Z_INDEX: f32 = 10.0;

pub struct AppUiCanvasEntityMap {
    pub root: Entity,
    pub fg: Entity,
    pub bg: Entity,
}

#[derive(Component)]
pub struct AppUiCanvas;

pub fn build_ui_canvas(commands: &mut Commands) -> AppUiCanvasEntityMap {
    let root_entity = commands
        .spawn((
            SpriteBundle {
                transform: Transform::from_xyz(0.0, 0.0, UI_CANVAS_Z_INDEX),
                sprite: Sprite { ..default() },
                ..default()
            },
            AppUiCanvas,
        ))
        .id();
    let mut bg_entity = Entity::PLACEHOLDER;
    let mut fg_entity = Entity::PLACEHOLDER;
    if let Some(mut entity_commands) = commands.get_entity(root_entity) {
        entity_commands.with_children(|parent| {
            bg_entity = parent
                .spawn(SpriteBundle {
                    transform: Transform::from_xyz(0.0, 0.0, UI_CANVAS_Z_INDEX + 0.1),
                    sprite: Sprite::default(),
                    ..default()
                })
                .id();
            fg_entity = parent
                .spawn(SpriteBundle {
                    transform: Transform::from_xyz(0.0, 0.0, UI_CANVAS_Z_INDEX + 0.2),
                    sprite: Sprite::default(),
                    ..default()
                })
                .id();
        });
    }
    AppUiCanvasEntityMap {
        root: root_entity,
        bg: bg_entity,
        fg: fg_entity,
    }
}

pub fn clear_ui_canvas(
    mut commands: Commands,
    ae_query: Query<Entity, With<AppUiCanvas>>,
    mut build_timer: ResMut<timer::AppUiBuildTimer>,
    mut refresh_timer: ResMut<timer::AppUiRefreshTimer>,
) {
    for ae_entity in ae_query.iter() {
        if let Some(entity_commands) = commands.get_entity(ae_entity) {
            entity_commands.despawn_recursive()
        }
    }
    build_timer.0.reset();
    refresh_timer.0.reset();
}

pub fn to_canvas_pos(window: &Query<&Window>, window_pos: Vec2) -> Vec2 {
    let window = window.single();
    let win_w = window.resolution.width();
    let win_h = window.resolution.height();
    return Vec2::new(window_pos.x, -window_pos.y) - Vec2::new(win_w / 2.0, -win_h / 2.0);
}
