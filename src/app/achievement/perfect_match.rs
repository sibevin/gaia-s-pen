use crate::{app::achievement::*, app::status};

pub struct AchievementDef;

impl AchievementDefBase for AchievementDef {
    fn code(&self) -> &str {
        "perfact_match"
    }
    fn name(&self) -> &str {
        "Impressionism"
    }
    fn color(&self) -> Color {
        Color::rgb(0.22, 0.60, 0.97)
    }
    fn description(&self) -> String {
        String::from("Get the 100% perfect match.")
    }
    fn check_done(&self, status: &ResMut<status::AppStatus>) -> (u32, u32, bool) {
        let current = status.match_point;
        let target = 1000;
        let is_done = current == target;
        (current, target, is_done)
    }
    fn progress_ui(&self) -> AchievementProgressUi {
        AchievementProgressUi::Bar
    }
}
