use super::*;
use crate::{app::anime_effect, app::interaction, app::ui};
use bevy_persistent::prelude::*;
use bevy_ui_navigation::NavRequestSystem;

const PAGE_CODE: &str = "settings_audio";
const PAGE_NAME: &str = "Settings";
const PAGE_ICON: &str = "gear";

pub struct Page;

impl PageBase for Page {
    fn code(&self) -> &str {
        PAGE_CODE
    }
    fn name(&self) -> &str {
        PAGE_NAME
    }
    fn icon(&self) -> &str {
        PAGE_ICON
    }
    fn state(&self) -> PageState {
        PageState::SettingsAudio
    }
    fn build(&self, app: &mut App) {
        app.add_systems(
            OnEnter(self.state()),
            (
                interaction::reset_default_focus,
                ui::clear_ui_canvas,
                page_enter,
            ),
        )
        .add_systems(
            Update,
            ((
                handle_ui_navigation,
                ui::handle_ui_mouse_unlock,
                ui::handle_ui_mouse_clicking,
                ui::handle_ui_mouse_dragging,
                ui::handle_ui_keyboard_lock,
                ui::handle_ui_keyboard_changing,
                ui::handle_ui_gamepad_lock,
                ui::handle_ui_gamepad_dpad_changing,
                ui::handle_ui_gamepad_axis_changing,
                interaction::handle_default_focus,
                ui::handle_ui_gamepad_modifier,
                ui::handle_ui_keyboard_modifier,
                ui::refresh_ui_canvas,
                handle_ui_events,
            )
                .after(NavRequestSystem),)
                .run_if(in_state(self.state())),
        )
        .add_systems(
            OnExit(self.state()),
            (
                anime_effect::clear_anime_effect,
                ui::clear_ui_canvas,
                ui::despawn_ui::<OnPage>,
            ),
        );
    }
}

#[derive(Component)]
struct InteractionDefaultFocus;

#[derive(Component)]
struct OnPage;

fn page_enter(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    settings: Res<Persistent<app::settings::Settings>>,
) {
    let se_slider_canvas = ui::create_ui_canvas(&mut commands);
    let bgm_slider_canvas = ui::create_ui_canvas(&mut commands);
    commands
        .spawn((build_page_layout(), OnPage))
        .with_children(|parent| {
            parent
                .spawn(NodeBundle {
                    style: Style {
                        width: Val::Percent(100.0),
                        flex_direction: FlexDirection::Column,
                        align_items: AlignItems::Center,
                        justify_content: JustifyContent::SpaceBetween,
                        ..default()
                    },
                    ..default()
                })
                .with_children(|parent| {
                    build_game_title(parent, &asset_server);
                    build_page_title(parent, &asset_server, PAGE_NAME, PAGE_ICON);
                    parent
                        .spawn(NodeBundle {
                            style: Style {
                                flex_grow: 1.0,
                                flex_direction: FlexDirection::Column,
                                align_items: AlignItems::Center,
                                justify_content: JustifyContent::Center,
                                ..default()
                            },
                            ..default()
                        })
                        .with_children(|parent| {
                            build_sep_title(parent, &asset_server, "BGM", "music-notes-fill");
                            ui::build_switch_btn(
                                parent,
                                &asset_server,
                                ButtonAction::Toggle(String::from("bgm")),
                                settings.is_enabled("bgm"),
                            );
                            ui::build_ui(
                                parent,
                                &asset_server,
                                ButtonAction::AppUiNav,
                                bgm_slider_canvas,
                                ui::AppUiInitParams::Slider {
                                    data: ui::AppUiTargetValuePair {
                                        target: String::from("bgm"),
                                        value: settings.get_value("bgm"),
                                    },
                                },
                            );
                            build_sep_title(parent, &asset_server, "SE", "waveform-fill");
                            ui::build_switch_btn(
                                parent,
                                &asset_server,
                                ButtonAction::Toggle(String::from("se")),
                                settings.is_enabled("se"),
                            );
                            parent
                                .spawn(NodeBundle {
                                    style: Style {
                                        align_items: AlignItems::Center,
                                        column_gap: ui::px_p(4.0),
                                        ..default()
                                    },
                                    ..default()
                                })
                                .with_children(|parent| {
                                    ui::build_ui(
                                        parent,
                                        &asset_server,
                                        ButtonAction::AppUiNav,
                                        se_slider_canvas,
                                        ui::AppUiInitParams::Slider {
                                            data: ui::AppUiTargetValuePair {
                                                target: String::from("se"),
                                                value: settings.get_value("se"),
                                            },
                                        },
                                    );
                                    ui::build_btn(
                                        parent,
                                        &asset_server,
                                        (
                                            ButtonAction::PlaySe,
                                            app::interaction::IaButton,
                                            Focusable::default(),
                                        ),
                                        Style {
                                            padding: UiRect::all(ui::px_p(ui::BTN_PADDING)),
                                            ..default()
                                        },
                                        None,
                                        Some("play"),
                                    );
                                });
                        });
                    build_settings_nav_bar(parent, &asset_server, PageState::SettingsAudio);
                });
        });
}

fn handle_ui_events(
    mut events: EventReader<ui::AppUiEvent>,
    mut settings: ResMut<Persistent<app::settings::Settings>>,
    audio_bgm_query: Query<&AudioSink, With<app::audio::AudioBgm>>,
    mut ui_query: Query<(Entity, &mut ui::AppUiData), With<ui::AppUiData>>,
    mut nav_requests: EventWriter<NavRequest>,
) {
    for event in events.read() {
        match event {
            ui::AppUiEvent::DataChanged { data } => {
                settings
                    .update(|settings| {
                        settings.set_value(data.target.as_str(), data.value as i8);
                    })
                    .expect("failed to update slider");
                if data.target == "bgm" {
                    if let Ok(sink) = audio_bgm_query.get_single() {
                        sink.set_volume(app::audio::to_volume(data.value));
                    }
                }
                ui::update_ui_value(&mut ui_query, data.clone());
            }
            ui::AppUiEvent::Lock { entity: _ } => {
                nav_requests.send(NavRequest::Lock);
            }
            ui::AppUiEvent::Unlock => {
                nav_requests.send(NavRequest::Unlock);
            }
            _ => (),
        }
    }
}

#[allow(clippy::too_many_arguments)]
fn handle_ui_navigation(
    mut commands: Commands,
    action_query: Query<(Entity, &mut ButtonAction), With<ButtonAction>>,
    mut switch_btn_query: Query<(&Parent, &mut UiImage, &mut ui::SwitchButton)>,
    mut nav_events: EventReader<NavEvent>,
    mut page_state: ResMut<NextState<PageState>>,
    mut settings: ResMut<Persistent<app::settings::Settings>>,
    mut ui_query: Query<(Entity, &mut ui::AppUiData), With<ui::AppUiData>>,
    audio_bgm_query: Query<&AudioSink, With<app::audio::AudioBgm>>,
    audio_se_asset: Res<app::audio::AudioSeAsset>,
    asset_server: Res<AssetServer>,
) {
    for event in nav_events.read() {
        match event {
            NavEvent::NoChanges { from, request } => match *request {
                NavRequest::Action => {
                    for (entity, action) in action_query.iter() {
                        if *from.first() == entity {
                            match action {
                                ButtonAction::Toggle(target) => {
                                    settings
                                        .update(|settings| {
                                            settings.toggle(target.as_ref());
                                        })
                                        .expect("failed to update boolean switch");
                                    let is_enabled = settings.is_enabled(target);
                                    ui::update_switch_btn_value(
                                        entity,
                                        &mut switch_btn_query,
                                        &asset_server,
                                        is_enabled,
                                    );
                                    if let Ok(sink) = audio_bgm_query.get_single() {
                                        if is_enabled {
                                            sink.play();
                                        } else {
                                            sink.pause();
                                        }
                                    }
                                }
                                ButtonAction::PlaySe => {
                                    app::audio::play_se(
                                        app::audio::AudioSe::Boom,
                                        &mut commands,
                                        &audio_se_asset,
                                        settings.as_ref(),
                                    );
                                }
                                ButtonAction::MoveToPage(state) => page_state.set(*state),
                                _ => (),
                            }
                        }
                    }
                }
                NavRequest::Unlock => {
                    ui::apply_ui_lock(None, &mut ui_query);
                }
                _ => (),
            },
            _ => (),
        }
    }
}
