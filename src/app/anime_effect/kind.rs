use crate::app::anime_effect::*;

pub mod circle_q;
pub mod line_q;

const ANIME_EFFECT_CANVAS_Z_INDEX: f32 = 1.0;

pub trait AnimeEffectKindBase {
    fn create(&self, commands: &mut Commands, param: AnimeEffectParam) -> Entity;
    fn draw(&self, commands: &mut Commands, ae: &mut AnimeEffect);
}

pub fn fetch_builder(kind: AnimeEffectKind) -> &'static dyn AnimeEffectKindBase {
    match kind {
        AnimeEffectKind::CircleQ => &kind::circle_q::AnimeEffectKindCircleQ,
        AnimeEffectKind::LineQ => &kind::line_q::AnimeEffectKindLineQ,
    }
}

fn noise_pos(pos: Vec2, radius: f32) -> Vec2 {
    let mut rng = thread_rng();
    let angle = rng.gen_range(0.0..2.0 * PI);
    let length = rng.gen_range(0.0..radius);
    pos + Vec2::new(angle.cos(), angle.sin()) * length
}
