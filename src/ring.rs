use egui::{Color32, Pos2};

pub struct Ring {
    pub center: Pos2,
    pub radius: f32,
    pub color: Color32,
}

impl Default for Ring {
    fn default() -> Self {
        Ring {
            center: Pos2::new(0.0, 0.0),
            radius: 100.0,
            color: Color32::WHITE,
        }
    }
}

impl Ring {
    pub fn new(center: Pos2, radius: f32, color: Color32) -> Ring {
        Ring {
            center,
            radius,
            color,
        }
    }
}
