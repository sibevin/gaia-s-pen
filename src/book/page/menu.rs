use crate::{app::anime_effect, app::theme::*, app::ui, book::page::*};
#[cfg(not(target_arch = "wasm32"))]
use bevy::app::AppExit;
use bevy_persistent::prelude::*;
use bevy_ui_navigation::{prelude::*, NavRequestSystem};

const PAGE_CODE: &str = "menu";
const PAGE_NAME: &str = "Menu";
const PAGE_ICON: &str = "arrow-left";

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
        PageState::Menu
    }
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(self.state()), page_enter)
            .add_systems(
                Update,
                (
                    handle_window_resize,
                    handle_menu_navigation.after(NavRequestSystem),
                )
                    .run_if(in_state(self.state())),
            )
            .add_systems(
                OnExit(self.state()),
                (anime_effect::clear_anime_effect, ui::despawn_ui::<OnPage>),
            );
    }
}

#[derive(Component)]
struct OnPage;

#[derive(Component)]
enum ButtonAction {
    MoveToPage(PageState),
    #[cfg(not(target_arch = "wasm32"))]
    Quit,
}

const MENU_PAGES: [&dyn PageBase; 4] = [
    &game::Page,
    &help::Page,
    &settings::audio::Page,
    &about::main::Page,
];

fn page_enter(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands
        .spawn((build_page_layout(), OnPage))
        .with_children(|parent| {
            parent
                .spawn(NodeBundle {
                    style: Style {
                        width: Val::Percent(100.0),
                        height: Val::Percent(100.0),
                        flex_direction: FlexDirection::Column,
                        align_items: AlignItems::Center,
                        justify_content: JustifyContent::Center,
                        margin: UiRect::bottom(ui::px_p(30.0)),
                        ..default()
                    },
                    ..default()
                })
                .with_children(|parent| {
                    parent
                        .spawn(NodeBundle {
                            style: Style {
                                flex_direction: FlexDirection::Row,
                                align_items: AlignItems::Center,
                                margin: UiRect::bottom(ui::px_p(12.0)),
                                ..default()
                            },
                            ..default()
                        })
                        .with_children(|parent| {
                            let icon = asset_server.load("images/app/logo.png");
                            parent.spawn(ImageBundle {
                                style: Style {
                                    width: Val::Px(80.0),
                                    height: Val::Px(80.0),
                                    margin: UiRect::right(ui::px_p(6.0)),
                                    ..default()
                                },
                                image: UiImage::new(icon),
                                ..default()
                            });
                            parent.spawn(
                                TextBundle::from_section(
                                    app::APP_NAME,
                                    TextStyle {
                                        font: asset_server.load(FONT_TITLE),
                                        font_size: ui::FONT_SIZE * 2.8,
                                        color: FG_COLOR,
                                    },
                                )
                                .with_style(Style {
                                    margin: UiRect::bottom(ui::px_p(4.0)),
                                    ..default()
                                }),
                            );
                        });
                    parent
                        .spawn(NodeBundle {
                            style: Style {
                                display: Display::Grid,
                                grid_template_columns: vec![GridTrack::fr(1.0)],
                                column_gap: Val::Px(ui::MENU_ENTRY_PADDING),
                                row_gap: Val::Px(ui::MENU_ENTRY_PADDING),
                                ..default()
                            },
                            ..default()
                        })
                        .with_children(|parent| {
                            for page_def in MENU_PAGES {
                                ui::build_menu_entry(
                                    parent,
                                    &asset_server,
                                    (
                                        ButtonAction::MoveToPage(page_def.state()),
                                        app::interaction::IaButton,
                                        Focusable::default(),
                                    ),
                                    page_def.name(),
                                    page_def.icon(),
                                );
                            }
                            #[cfg(not(target_arch = "wasm32"))]
                            ui::build_menu_entry(
                                parent,
                                &asset_server,
                                (
                                    ButtonAction::Quit,
                                    app::interaction::IaButton,
                                    Focusable::default(),
                                ),
                                "Quit",
                                "sign-out",
                            );
                        });
                });
        });
}

fn handle_menu_navigation(
    mut actions: Query<&mut ButtonAction>,
    mut events: EventReader<NavEvent>,
    mut page_state: ResMut<NextState<PageState>>,
    mut settings: ResMut<Persistent<app::settings::Settings>>,
    #[cfg(not(target_arch = "wasm32"))] mut app_exit_events: EventWriter<AppExit>,
) {
    events.nav_iter().activated_in_query_foreach_mut(
        &mut actions,
        |mut action| match &mut *action {
            ButtonAction::MoveToPage(state) => {
                match state {
                    PageState::Game => {
                        if settings.is_enabled("first") {
                            settings.toggle("first");
                            page_state.set(PageState::Help)
                        } else {
                            page_state.set(PageState::Game)
                        }
                    }
                    _ => page_state.set(*state),
                };
            }
            #[cfg(not(target_arch = "wasm32"))]
            ButtonAction::Quit => app_exit_events.send(AppExit),
        },
    );
}

fn handle_window_resize(window: Query<&Window>) {
    let window = window.single();
    let _width = window.resolution.width();
    let _height = window.resolution.height();
    // dbg!(width, height);
}
