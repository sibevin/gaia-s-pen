use crate::{app::anime_effect::*, app::theme::*, app::ui};
use bevy::prelude::*;
use bevy_ui_navigation::{
    prelude::{FocusState, Focusable},
    NavRequestSystem,
};

#[derive(Component)]
pub struct IaButton;

#[derive(Component)]
pub struct IaSwitch;

#[derive(Component)]
pub struct IaSlider;

#[derive(Component)]
pub struct IaLink;

#[derive(Component)]
pub struct IaPanel;

#[derive(Component)]
pub struct IaAnimeEffect;

pub struct InteractionPlugin;

impl Plugin for InteractionPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                update_button_interaction,
                update_switch_interaction,
                update_slider_interaction,
                update_link_interaction,
            )
                .after(NavRequestSystem),
        );
    }
}

const WAVE_START_W: f32 = ui::FONT_SIZE * 0.3;
const WAVE_END_W: f32 = WAVE_START_W * 0.4;
const WAVE_H: f32 = WAVE_START_W * 0.4;

#[derive(Default)]
struct FocusTarget {
    pub pos: Vec2,
    pub size: Vec2,
}

fn fetch_focus_target(
    window: &Query<&Window>,
    g_trans: &GlobalTransform,
    node: &Node,
) -> FocusTarget {
    let g_pos = Vec2::new(g_trans.translation().x, -g_trans.translation().y);
    let window = window.single();
    let win_w = window.resolution.width();
    let win_h = window.resolution.height();
    FocusTarget {
        pos: g_pos - Vec2::new(win_w / 2.0, -win_h / 2.0),
        size: Vec2::new(node.size().x, node.size().y),
    }
}

type FocusableButton = (Changed<Focusable>, With<IaButton>);

fn update_button_interaction(
    mut commands: Commands,
    mut focusables: Query<(&Focusable, &GlobalTransform, &Node), FocusableButton>,
    ae_query: Query<Entity, With<IaAnimeEffect>>,
    mut ae_status: ResMut<AnimeEffectStatus>,
    window: Query<&Window>,
) {
    let mut target: Option<FocusTarget> = None;
    for (focus, g_trans, node) in focusables.iter_mut() {
        if matches!(focus.state(), FocusState::Focused) {
            target = Some(fetch_focus_target(&window, g_trans, node));
        } else {
            for ae_entity in ae_query.iter() {
                despawn_anime_effect(ae_entity, &mut ae_status);
            }
        }
    }
    if let Some(target) = target {
        if target.size.x > 0.0 && target.size.y > 0.0 {
            if target.size.x > target.size.y * 1.5 {
                let size_x = target.size.x * 0.9;
                let size_y = target.size.y * 0.5;
                insert_anime_effect(
                    &mut commands,
                    AnimeEffectParam {
                        kind: AnimeEffectKind::LineQ,
                        color: BG_COLOR.with_l(0.8),
                        pos_1: Vec2::new(
                            target.pos.x - size_x / 2.0,
                            target.pos.y - size_y / 2.0 - WAVE_H,
                        ),
                        pos_2: Vec2::new(
                            target.pos.x + size_x / 2.0,
                            target.pos.y - size_y / 2.0 + WAVE_H,
                        ),
                        width_start: WAVE_START_W,
                        width_end: WAVE_END_W,
                    },
                    IaAnimeEffect,
                );
            } else {
                insert_anime_effect(
                    &mut commands,
                    AnimeEffectParam {
                        kind: AnimeEffectKind::CircleQ,
                        color: BG_COLOR.with_l(0.8),
                        pos_1: Vec2::new(target.pos.x, target.pos.y),
                        pos_2: Vec2::new(
                            target.pos.x + target.size.x / 2.0,
                            target.pos.y + target.size.y / 2.0,
                        ),
                        width_start: WAVE_START_W,
                        width_end: WAVE_END_W,
                    },
                    IaAnimeEffect,
                );
            }
        }
    }
}

type FocusableSwitch = (Changed<Focusable>, With<IaSwitch>);

fn update_switch_interaction(
    mut commands: Commands,
    mut focusables: Query<(&Focusable, &GlobalTransform, &Node), FocusableSwitch>,
    ae_query: Query<Entity, With<IaAnimeEffect>>,
    mut ae_status: ResMut<AnimeEffectStatus>,
    window: Query<&Window>,
) {
    let mut target: Option<FocusTarget> = None;
    for (focus, g_trans, node) in focusables.iter_mut() {
        if matches!(focus.state(), FocusState::Focused) {
            target = Some(fetch_focus_target(&window, g_trans, node));
        } else {
            for ae_entity in ae_query.iter() {
                despawn_anime_effect(ae_entity, &mut ae_status);
            }
        }
    }
    if let Some(target) = target {
        let size_y = target.size.y * 0.3;
        insert_anime_effect(
            &mut commands,
            AnimeEffectParam {
                kind: AnimeEffectKind::LineQ,
                color: BG_COLOR.with_l(0.8),
                pos_1: Vec2::new(
                    target.pos.x - target.size.x / 2.0,
                    target.pos.y - size_y / 2.0 - WAVE_H,
                ),
                pos_2: Vec2::new(
                    target.pos.x + target.size.x / 2.0,
                    target.pos.y - size_y / 2.0 + WAVE_H,
                ),
                width_start: WAVE_START_W,
                width_end: WAVE_END_W,
            },
            IaAnimeEffect,
        );
    }
}

type FocusableSlider = (Changed<Focusable>, With<IaSlider>);

fn update_slider_interaction(
    mut commands: Commands,
    mut focusables: Query<(&Focusable, &GlobalTransform, &Node), FocusableSlider>,
    ae_query: Query<Entity, With<IaAnimeEffect>>,
    mut ae_status: ResMut<AnimeEffectStatus>,
    window: Query<&Window>,
) {
    let mut target: Option<FocusTarget> = None;
    for (focus, g_trans, node) in focusables.iter_mut() {
        if matches!(focus.state(), FocusState::Focused) {
            target = Some(fetch_focus_target(&window, g_trans, node));
        } else {
            for ae_entity in ae_query.iter() {
                despawn_anime_effect(ae_entity, &mut ae_status);
            }
        }
    }
    if let Some(target) = target {
        let size_x = target.size.x * 0.95;
        let size_y = target.size.y * 0.35;
        insert_anime_effect(
            &mut commands,
            AnimeEffectParam {
                kind: AnimeEffectKind::LineQ,
                color: BG_COLOR.with_l(0.8),
                pos_1: Vec2::new(
                    target.pos.x - size_x / 2.0,
                    target.pos.y - size_y / 2.0 - WAVE_H,
                ),
                pos_2: Vec2::new(
                    target.pos.x + size_x / 2.0,
                    target.pos.y - size_y / 2.0 + WAVE_H,
                ),
                width_start: WAVE_START_W,
                width_end: WAVE_END_W,
            },
            IaAnimeEffect,
        );
    }
}

type FocusableLink = (Changed<Focusable>, With<IaLink>);

fn update_link_interaction(
    mut commands: Commands,
    mut focusables: Query<(&Focusable, &GlobalTransform, &Node), FocusableLink>,
    ae_query: Query<Entity, With<IaAnimeEffect>>,
    mut ae_status: ResMut<AnimeEffectStatus>,
    window: Query<&Window>,
) {
    let mut target: Option<FocusTarget> = None;
    for (focus, g_trans, node) in focusables.iter_mut() {
        if matches!(focus.state(), FocusState::Focused) {
            target = Some(fetch_focus_target(&window, g_trans, node));
        } else {
            for ae_entity in ae_query.iter() {
                despawn_anime_effect(ae_entity, &mut ae_status);
            }
        }
    }
    if let Some(target) = target {
        let size_y = target.size.y * 0.8;
        insert_anime_effect(
            &mut commands,
            AnimeEffectParam {
                kind: AnimeEffectKind::LineQ,
                color: FG_COLOR.with_l(0.8),
                pos_1: Vec2::new(
                    target.pos.x - target.size.x / 2.0,
                    target.pos.y - size_y / 2.0 - WAVE_H,
                ),
                pos_2: Vec2::new(
                    target.pos.x + target.size.x / 2.0,
                    target.pos.y - size_y / 2.0 + WAVE_H,
                ),
                width_start: WAVE_START_W,
                width_end: WAVE_END_W,
            },
            IaAnimeEffect,
        );
    }
}
