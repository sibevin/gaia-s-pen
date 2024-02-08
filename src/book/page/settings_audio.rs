use crate::{app::anime_effect, app::interaction, app::ui, book::page::*};
use bevy::window::WindowMode;
use bevy_persistent::prelude::*;
use bevy_ui_navigation::{prelude::*, NavRequestSystem};

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
            (interaction::reset_default_focus, page_enter),
        )
        .add_systems(
            Update,
            (
                ui::refresh_slider_display,
                handle_slider_change,
                (
                    handle_ui_navigation,
                    ui::handle_slider_mouse_clicking,
                    ui::handle_slider_mouse_dragging,
                    interaction::handle_default_focus,
                )
                    .after(NavRequestSystem),
            )
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

#[derive(Component, Debug)]
enum ButtonAction {
    BackToMainMenu,
    Toggle(String),
    SetValue(String),
    PlaySe,
}

fn page_enter(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    settings: Res<Persistent<app::settings::Settings>>,
) {
    let se_slider_canvas = ui::build_ui_canvas(&mut commands);
    let bgm_slider_canvas = ui::build_ui_canvas(&mut commands);
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
                                margin: UiRect::top(ui::px_p(24.0)),
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
                            ui::build_slider_ui(
                                bgm_slider_canvas,
                                parent,
                                &asset_server,
                                ButtonAction::SetValue(String::from("bgm")),
                                "bgm",
                                settings.get_value("bgm"),
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
                                    ui::build_slider_ui(
                                        se_slider_canvas,
                                        parent,
                                        &asset_server,
                                        ButtonAction::SetValue(String::from("se")),
                                        "se",
                                        settings.get_value("se"),
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
                    ui::build_icon_btn(
                        parent,
                        &asset_server,
                        (
                            ButtonAction::BackToMainMenu,
                            app::interaction::IaButton,
                            Focusable::default(),
                            app::interaction::IaDefaultFocus,
                        ),
                        Style {
                            align_self: AlignSelf::Start,
                            ..default()
                        },
                        "arrow-left-bold_x1.5",
                    );
                });
        });
}

fn handle_slider_change(
    mut events: EventReader<ui::SliderChangedEvent>,
    mut settings: ResMut<Persistent<app::settings::Settings>>,
    audio_bgm_query: Query<&AudioSink, With<app::audio::AudioBgm>>,
) {
    for event in events.read() {
        settings
            .update(|settings| {
                settings.set_value(event.target.as_str(), event.value as i8);
            })
            .expect("failed to update slider");
        if event.target == "bgm" {
            if let Ok(sink) = audio_bgm_query.get_single() {
                sink.set_volume(app::audio::to_volume(event.value));
            }
        }
    }
}

#[allow(clippy::too_many_arguments)]
fn handle_ui_navigation(
    mut commands: Commands,
    mut action_query: Query<(&mut ButtonAction, &Children), Without<app::interaction::IaSlider>>,
    mut switch_btn_query: Query<(Entity, &mut UiImage, &mut ui::SwitchButton)>,
    mut events: EventReader<NavEvent>,
    mut page_state: ResMut<NextState<PageState>>,
    mut settings: ResMut<Persistent<app::settings::Settings>>,
    mut window_query: Query<&mut Window>,
    audio_bgm_query: Query<&AudioSink, With<app::audio::AudioBgm>>,
    audio_se_asset: Res<app::audio::AudioSeAsset>,
    asset_server: Res<AssetServer>,
) {
    events.nav_iter().activated_in_query_foreach_mut(
        &mut action_query,
        |(mut action, children)| match &mut *action {
            ButtonAction::Toggle(target) => {
                ui::update_switch_btn_display(children, &mut switch_btn_query, &asset_server);
                settings
                    .update(|settings| {
                        settings.toggle(target.as_ref());
                    })
                    .expect("failed to update boolean switch");
                let is_enabled = settings.is_enabled(target);
                if target == "fullscreen" {
                    let mut window = window_query.single_mut();
                    if is_enabled {
                        window.mode = WindowMode::Fullscreen
                    } else {
                        window.mode = WindowMode::Windowed
                    }
                } else if target == "bgm" {
                    if let Ok(sink) = audio_bgm_query.get_single() {
                        if is_enabled {
                            sink.play();
                        } else {
                            sink.pause();
                        }
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
            ButtonAction::BackToMainMenu => page_state.set(PageState::Menu),
            _ => (),
        },
    );
}
