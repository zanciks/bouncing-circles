use eframe::egui::{Color32, Pos2, Vec2};

pub struct Ball {
    pub position: Pos2,
    pub velocity: Vec2,
    pub color: Color32,
}

impl Default for Ball {
    fn default() -> Ball {
        Ball {
            position: Pos2::new(840.0, 0.0),
            velocity: Vec2::new(0.0, 0.0),
            color: Color32::WHITE,
        }
    }
}

impl Ball {
    pub fn new(position: Pos2, velocity: Vec2, color: Color32) -> Ball {
        Ball {
            position,
            velocity,
            color,
        }
    }
}
