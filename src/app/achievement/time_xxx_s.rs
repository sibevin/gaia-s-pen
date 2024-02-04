use crate::{app::achievement::*, app::status};

pub struct AchievementDef;

impl AchievementDefBase for AchievementDef {
    fn code(&self) -> &str {
        "time_xxx_s"
    }
    fn name(&self) -> &str {
        "Cubism"
    }
    fn color(&self) -> Color {
        Color::rgb(0.22, 0.60, 0.97)
    }
    fn description(&self) -> String {
        String::from("Use only 10s to complete the drawing")
    }
    fn check_done(&self, status: &ResMut<status::AppStatus>) -> (u32, u32, bool) {
        let current = status.time;
        let target = 1000;
        let is_done = current < target;
        (current, target, is_done)
    }
    fn progress_ui(&self) -> AchievementProgressUi {
        AchievementProgressUi::Bar
    }
}
