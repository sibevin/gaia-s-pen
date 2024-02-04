use bevy::prelude::*;
use chrono::Local;

#[derive(PartialEq, Default, Debug)]
pub enum StatusChain {
    #[default]
    None,
    Hyper,
    Control,
}

#[derive(Resource, Default, Debug)]
pub struct AppStatus {
    pub player_name: String,
    pub highlight_uid: String,
    pub done_achievements: Vec<String>,
    pub in_modified_sensitivity: bool,
    pub time: u32,
    pub score: u32,
    pub undo_count: u32,
    pub repaint_count: u32,
    pub match_point: u32,
    started_at: String,
    ended_at: String,
}

impl AppStatus {
    pub fn uid(&self) -> &str {
        &self.started_at
    }

    pub fn mark_timeline(&mut self, timeline_type: &str) {
        let now_dt = Local::now().format("%Y-%m-%d_%H:%M:%S%.9f").to_string();
        match timeline_type {
            "started" => {
                self.started_at = now_dt;
            }
            "ended" => {
                self.ended_at = now_dt;
            }
            _ => panic!("Invalid timeline type"),
        }
    }

    pub fn reset(&mut self) {
        let highlight_uid = self.highlight_uid.clone();
        *self = self::default();
        self.highlight_uid = highlight_uid;
        self.mark_timeline("started");
    }
}
