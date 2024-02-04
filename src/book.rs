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
    About,
    Help,
    Dev,
}

pub const PAGES: [&dyn page::PageBase; 6] = [
    &page::menu::Page,
    &page::game::Page,
    &page::settings::Page,
    &page::help::Page,
    &page::about::Page,
    &page::dev::Page,
];
