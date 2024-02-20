use bevy::prelude::*;

mod page;
mod plugin;

pub use plugin::BookPlugin;

#[derive(Clone, Copy, Default, Eq, PartialEq, Debug, Hash, States)]
pub enum PageState {
    #[default]
    Menu,
    Game,
    Settings,
    SettingsAudio,
    SettingsControl,
    SettingsDisplay,
    About,
    Help,
    Dev,
}

pub const PAGES: [&dyn page::PageBase; 8] = [
    &page::menu::Page,
    &page::game::Page,
    &page::help::Page,
    &page::about::Page,
    &page::dev::Page,
    &page::settings::audio::Page,
    &page::settings::control::Page,
    &page::settings::display::Page,
];
