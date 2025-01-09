use crate::dot::Dot;
use eframe::egui::{
    CentralPanel, Color32, Context, Pos2, Rect, Sense, Shape, Stroke, Ui, Vec2, Window,
};
use wasm_bindgen::prelude::*;

pub struct Simulation {
    paused: bool,
    simulation_area: Rect,

    dots: Vec<Dot>,
    dot_size: f32,
    gravity: f32,
    control_point: Pos2,

    new_col: Color32,
}

impl Default for Simulation {
    fn default() -> Simulation {
        Simulation {
            paused: true,
            simulation_area: Rect::ZERO,

            dots: vec![],
            dot_size: 5.0,
            gravity: 5.0,
            control_point: Pos2::new(91.5, 91.5),

            new_col: Color32::WHITE,
        }
    }
}

impl eframe::App for Simulation {
    fn update(&mut self, ctx: &Context, _frame: &mut eframe::Frame) {
        self.settings_window(ctx);

        let screen_rect = ctx.screen_rect();
        let width = screen_rect.width();
        let height = screen_rect.height();

        let size = (width * 0.95).min(height * 0.95);
        self.simulation_area = Rect::from_center_size(screen_rect.center(), Vec2::splat(size));

        CentralPanel::default().show(ctx, |ui| {
            if !self.paused {
                ctx.input(|i| self.physics_update(i.stable_dt));
                ctx.request_repaint()
            }
            self.draw_update(ui);
        });
    }
}

impl Simulation {
    fn draw_update(&self, ui: &mut egui::Ui) {
        let background_color = Color32::from_black_alpha(200);
        ui.painter()
            .rect_filled(self.simulation_area, 0.0, background_color);

        for dot in &self.dots {
            ui.painter()
                .circle_filled(dot.position, self.dot_size, dot.color);
        }
    }
    fn physics_update(&mut self, delta: f32) {
        for dot in &mut self.dots {
            let dot_bottom = dot.position.y + self.dot_size;

            dot.velocity += Vec2::new(0.0, self.gravity * delta);
            dot.position += dot.velocity;

            if dot_bottom > self.simulation_area.max.y {
                playSound("/app/dink.mp3");
                dot.position.y = self.simulation_area.max.y - 2.0 * self.dot_size;
                dot.velocity.y *= -1.0;
            }
        }
    }
    fn settings_window(&mut self, ctx: &Context) {
        Window::new("Settings").show(ctx, |ui| {
            ui.horizontal(|ui| {
                if ui.button("Pause/Unpause").clicked() {
                    self.paused = !self.paused
                };
                ui.label(format!(
                    "{}",
                    match self.paused {
                        false => "Playing",
                        true => "Paused",
                    }
                ))
            });

            ui.add(egui::Slider::new(&mut self.dot_size, 1.0..=10.0).text("Dot Size"));
            ui.add(egui::Slider::new(&mut self.gravity, 1.0..=10.0).text("Gravity"));
            ui.shrink_width_to_current();
            ui.collapsing("Spawn new dot", |ui| {
                let test = self.position_picker(ui);
                ui.horizontal(|ui| {
                    if ui.button("Spawn").clicked() {
                        let scale = self.simulation_area.width() / test;
                        self.dots.push(Dot::new(
                            (self.control_point * scale)
                                + self.simulation_area.left_top().to_vec2(),
                            Vec2::ZERO,
                            self.new_col,
                        ));
                    }
                    ui.color_edit_button_srgba(&mut self.new_col);
                });
            });
        });
    }
    fn position_picker(&mut self, ui: &mut Ui) -> f32 {
        let width = ui.available_width();
        let (response, painter) = ui.allocate_painter(Vec2::splat(width), Sense::click_and_drag());
        let to_screen = emath::RectTransform::from_to(
            Rect::from_min_size(Pos2::ZERO, response.rect.size()),
            response.rect,
        );
        let control_point_radius = 5.0;

        let background_color = Color32::from_black_alpha(200);
        painter.rect_filled(response.rect, 0.0, background_color);

        let control_point_shape: Shape = {
            let size = Vec2::splat(2.0 * control_point_radius);

            let point_in_screen = to_screen.transform_pos(self.control_point);
            let point_rect = Rect::from_center_size(point_in_screen, size);
            let point_id = response.id.with(0);
            let point_response = ui.interact(point_rect, point_id, Sense::drag());

            self.control_point += point_response.drag_delta();
            self.control_point = to_screen.from().clamp(self.control_point);

            let point_in_screen = to_screen.transform_pos(self.control_point);
            let mut stroke = Stroke::new(1.0, self.new_col);

            if point_response.hovered() {
                stroke.color = stroke.color.lerp_to_gamma(Color32::WHITE, 0.5);
            }

            Shape::circle_stroke(point_in_screen, control_point_radius, stroke)
        };

        painter.add(control_point_shape);

        width
    }
}

#[wasm_bindgen]
extern "C" {
    fn playSound(filePath: &str);
}
