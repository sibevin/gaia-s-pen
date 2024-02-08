use crate::app::ui::*;
use bevy::input;

#[derive(Component)]
pub struct Slider {
    target: String,
    value: u8,
    canvas_em: AppUiCanvasEntityMap,
}

#[derive(Component)]
pub struct SliderText;

#[derive(Event)]
pub struct SliderChangedEvent {
    pub target: String,
    pub value: u8,
}

const SLIDER_BAR_H: f32 = FONT_SIZE * 0.5;
const SLIDER_BAR_W: f32 = FONT_SIZE * 5.0;
const SLIDER_BAR_B: f32 = FONT_SIZE * 0.1;
const SLIDER_TEXT_W: f32 = FONT_SIZE * 1.8;
const SLIDER_PL: f32 = FONT_SIZE * 0.5;
const SLIDER_W: f32 = SLIDER_PL + SLIDER_BAR_W + SLIDER_TEXT_W;
const SLIDER_H: f32 = FONT_SIZE * 2.0;

pub fn build_slider_ui(
    canvas_em: AppUiCanvasEntityMap,
    parent: &mut ChildBuilder,
    asset_server: &Res<AssetServer>,
    bundle: impl Bundle,
    target: &str,
    init_value: u8,
) -> Entity {
    parent
        .spawn((
            ButtonBundle {
                style: Style {
                    width: Val::Px(SLIDER_W),
                    height: Val::Px(SLIDER_H),
                    justify_content: JustifyContent::End,
                    align_items: AlignItems::Center,
                    ..default()
                },
                background_color: BTN_BG.into(),
                ..default()
            },
            bundle,
            app::interaction::IaSlider,
            Slider {
                target: String::from(target),
                value: init_value,
                canvas_em,
            },
            Focusable::default(),
        ))
        .with_children(|parent| {
            parent.spawn((
                TextBundle::from_section(
                    format!("{}", init_value),
                    TextStyle {
                        font: asset_server.load(FONT),
                        font_size: FONT_SIZE,
                        color: FG_COLOR,
                    },
                ),
                SliderText,
            ));
        })
        .id()
}

pub fn refresh_slider_display(
    mut commands: Commands,
    slider_query: Query<(&GlobalTransform, &Children, &Slider)>,
    mut slider_text_query: Query<(Entity, &mut Text, &SliderText)>,
    window: Query<&Window>,
    mut delay_timer: ResMut<timer::AppUiBuildTimer>,
    mut refresh_timer: ResMut<timer::AppUiRefreshTimer>,
    time: Res<Time>,
) {
    if delay_timer.0.tick(time.delta()).just_finished() {
        for (g_trans, _, slider) in slider_query.iter() {
            if let Some(mut entity_commands) = commands.get_entity(slider.canvas_em.bg) {
                entity_commands.despawn_descendants();
                entity_commands.with_children(|parent| {
                    let (bar_start_pos, bar_end_pos) = fetch_bar_pos(&window, &g_trans);
                    let mut path_builder = PathBuilder::new();
                    path_builder.move_to(bar_start_pos);
                    path_builder.line_to(bar_end_pos);
                    parent.spawn((
                        ShapeBundle {
                            path: path_builder.build(),
                            ..default()
                        },
                        Stroke::new(SECONDARY_COLOR, SLIDER_BAR_H),
                    ));
                    let circle = shapes::Circle {
                        radius: SLIDER_BAR_H / 2.0,
                        center: bar_start_pos,
                    };
                    let geo_builder = GeometryBuilder::new().add(&circle);
                    parent.spawn((
                        ShapeBundle {
                            path: geo_builder.build(),
                            ..default()
                        },
                        Fill::color(SECONDARY_COLOR),
                    ));
                    let circle = shapes::Circle {
                        radius: SLIDER_BAR_H / 2.0,
                        center: bar_end_pos,
                    };
                    let geo_builder = GeometryBuilder::new().add(&circle);
                    parent.spawn((
                        ShapeBundle {
                            path: geo_builder.build(),
                            ..default()
                        },
                        Fill::color(SECONDARY_COLOR),
                    ));
                });
            }
        }
    }
    if refresh_timer.0.tick(time.delta()).just_finished() {
        for (g_trans, children, slider) in slider_query.iter() {
            for child in children {
                for (bar_entity, mut bar_text, _) in slider_text_query.iter_mut() {
                    if *child == bar_entity {
                        bar_text.sections[0].value = format!("{}", slider.value);
                        break;
                    }
                }
            }
            if let Some(mut entity_commands) = commands.get_entity(slider.canvas_em.fg) {
                entity_commands.despawn_descendants();
                entity_commands.with_children(|parent| {
                    let (bar_start_pos, bar_end_pos) = fetch_bar_pos(&window, &g_trans);
                    let bar_thumb_pos = fetch_thumb_pos(&slider, bar_start_pos, bar_end_pos);
                    let mut path_builder = PathBuilder::new();
                    path_builder.move_to(bar_start_pos);
                    path_builder.line_to(bar_thumb_pos);
                    parent.spawn((
                        ShapeBundle {
                            path: path_builder.build(),
                            ..default()
                        },
                        Stroke::new(FG_COLOR, SLIDER_BAR_H - SLIDER_BAR_B * 2.0),
                    ));
                    let circle = shapes::Circle {
                        radius: SLIDER_BAR_H / 2.0 - SLIDER_BAR_B,
                        center: bar_start_pos,
                    };
                    let geo_builder = GeometryBuilder::new().add(&circle);
                    parent.spawn((
                        ShapeBundle {
                            path: geo_builder.build(),
                            ..default()
                        },
                        Fill::color(FG_COLOR),
                    ));
                    let circle = shapes::Circle {
                        radius: SLIDER_BAR_H / 2.0 - SLIDER_BAR_B,
                        center: bar_thumb_pos,
                    };
                    let geo_builder = GeometryBuilder::new().add(&circle);
                    parent.spawn((
                        ShapeBundle {
                            path: geo_builder.build(),
                            ..default()
                        },
                        Fill::color(FG_COLOR),
                    ));
                });
            }
        }
    }
}

const SLIDER_MOUSE_MOVING_RATIO: f32 = 0.5;

pub fn handle_slider_mouse_clicking(
    mut slider_clicking_query: Query<
        (&Interaction, &GlobalTransform, &mut Slider),
        Changed<Interaction>,
    >,
    window: Query<&Window>,
    cursor_data: Res<app::cursor::AppCursorData>,
    mut event_writer: EventWriter<SliderChangedEvent>,
) {
    for (interaction, &g_trans, mut slider) in slider_clicking_query.iter_mut() {
        if *interaction == Interaction::Pressed {
            let (bar_start_pos, _) = fetch_bar_pos(&window, &g_trans);
            slider.value = ((cursor_data.canvas_pos.x - bar_start_pos.x) / SLIDER_BAR_W * 100.0)
                .clamp(0.0, 100.0) as u8;
            event_writer.send(SliderChangedEvent {
                target: slider.target.clone(),
                value: slider.value,
            })
        }
    }
}

pub fn handle_slider_mouse_dragging(
    mut slider_moving_query: Query<(&Interaction, &mut Slider)>,
    mut mouse_motion_events: EventReader<input::mouse::MouseMotion>,
    mut event_writer: EventWriter<SliderChangedEvent>,
) {
    for (interaction, mut slider) in slider_moving_query.iter_mut() {
        if *interaction == Interaction::Pressed {
            let motion_events = mouse_motion_events.read().collect::<Vec<_>>();
            if let Some(motion_event) = motion_events.iter().rev().take(3).next() {
                slider.value = (slider.value as i8
                    + (motion_event.delta.x * SLIDER_MOUSE_MOVING_RATIO) as i8)
                    .clamp(0, 100) as u8;
                event_writer.send(SliderChangedEvent {
                    target: slider.target.clone(),
                    value: slider.value,
                })
            }
        }
    }
}

fn fetch_bar_pos(window: &Query<&Window>, g_trans: &GlobalTransform) -> (Vec2, Vec2) {
    let world_pos = Vec2::new(g_trans.translation().x, g_trans.translation().y);
    let g_pos = builder::to_canvas_pos(&window, world_pos);
    let bar_start_pos = Vec2::new(g_pos.x + SLIDER_PL - SLIDER_W / 2.0, g_pos.y);
    let bar_end_pos = Vec2::new(bar_start_pos.x + SLIDER_BAR_W, bar_start_pos.y);
    (bar_start_pos, bar_end_pos)
}

fn fetch_thumb_pos(slider: &Slider, bar_start_pos: Vec2, bar_end_pos: Vec2) -> Vec2 {
    let ratio = slider.value as f32 / 100.0;
    bar_start_pos + (bar_end_pos - bar_start_pos) * ratio
}
