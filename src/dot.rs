use eframe::egui::{Color32, Pos2, Vec2};

#[derive(PartialEq, Clone, Copy)]
pub struct Dot {
    pub position: Pos2,
    pub velocity: Vec2,
    pub color: Color32,
}

impl Default for Dot {
    fn default() -> Dot {
        Dot {
            position: Pos2::new(0.0, 0.0),
            velocity: Vec2::new(0.0, 0.0),
            color: Color32::WHITE,
        }
    }
}

impl Dot {
    pub fn new(position: Pos2, velocity: Vec2, color: Color32) -> Dot {
        Dot {
            position,
            velocity,
            color,
        }
    }
}
