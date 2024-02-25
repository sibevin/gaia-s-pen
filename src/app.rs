pub mod achievement;
pub mod anime_effect;
pub mod audio;
pub mod cursor;
pub mod cursor_icon;
pub mod interaction;
pub mod key_binding;
pub mod leaderboard;
pub mod plugin;
pub mod screenshot;
pub mod settings;
pub mod startup;
pub mod status;
pub mod theme;
pub mod timer;
pub mod ui;

pub use plugin::AppPlugin;
pub use startup::startup;

pub const WINDOW_W: f32 = 1280.0;
pub const WINDOW_H: f32 = 720.0;

pub const APP_CODE: &str = "gaia_s_pen";
pub const APP_NAME: &str = "Gaia's Pen";
pub const APP_SLOGAN: &str = "Use the pen of the goddess Gaia to create your own plants";
pub const APP_ITCH_URL: &str = "https://sibevin.itch.io/gaia-s-pen";
pub const APP_GITHUB_URL: &str = "https://github.com/sibevin/gaia-s-pen";
