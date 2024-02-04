use crate::{app::achievement::*, app::status};

pub struct AchievementDef;

impl AchievementDefBase for AchievementDef {
    fn code(&self) -> &str {
        "zero_undo"
    }
    fn name(&self) -> &str {
        "Expressionism"
    }
    fn color(&self) -> Color {
        Color::rgb(0.22, 0.60, 0.97)
    }
    fn description(&self) -> String {
        String::from("Finish the drawing without using the undo")
    }
    fn check_done(&self, status: &ResMut<status::AppStatus>) -> (u32, u32, bool) {
        let current = status.undo_count;
        let target = 0;
        let is_done = current == target;
        (current, target, is_done)
    }
    fn progress_ui(&self) -> AchievementProgressUi {
        AchievementProgressUi::Dots
    }
}
