use crate::ball::Ball;
use eframe::egui::{CentralPanel, Color32, Pos2, Rect, Vec2, Window, Context, Shape, Sense, Ui};
use log::info;

pub struct Simulation {
    paused: bool,
    balls: Vec<Ball>,
    ball_size: f32,
    gravity: f32,
    control_point: Pos2,

    new_col: Color32,
}

impl Default for Simulation {
    fn default() -> Simulation {
        Simulation {
            paused: true,
            balls: vec![],
            ball_size: 5.0,
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
        CentralPanel::default().show(ctx, |ui| {
            if !self.paused {
                ctx.input(|i| self.physics_update(i.stable_dt, screen_rect));
                ctx.request_repaint()
            }
            self.draw_update(ui);
        });
    }
}

impl Simulation {
    fn draw_update(&self, ui: &mut egui::Ui) {
        for ball in &self.balls {
            ui.painter()
                .circle_filled(ball.position, self.ball_size, ball.color);
        }
    }
    fn physics_update(&mut self, delta: f32, screen_rect: Rect) {
        for ball in &mut self.balls {
            let _ball_left = ball.position.x - self.ball_size;
            let _ball_right = ball.position.x + self.ball_size;
            let ball_bottom = ball.position.y + self.ball_size;
            let _ball_top = ball.position.y - self.ball_size;

            ball.velocity += Vec2::new(0.0, self.gravity * delta);
            ball.position += ball.velocity;
            if ball_bottom > screen_rect.max.y {
                ball.position.y = screen_rect.max.y - self.ball_size;
                ball.velocity.y *= -1.0;
            }
        }
    }
    fn settings_window(&mut self, ctx: &Context) {
        Window::new("Settings").movable(false).show(ctx, |ui| {
            if ui.button("Pause/Unpause").clicked() {
                self.paused = !self.paused
            }
            ui.add(egui::Slider::new(&mut self.ball_size, 1.0..=10.0).text("Ball Size"));
            ui.add(egui::Slider::new(&mut self.gravity, 1.0..=10.0).text("Gravity"));
            ui.shrink_width_to_current();
            ui.collapsing("Spawn ball", |ui| {
                let test = self.position_picker(ui);
                ui.horizontal(|ui| {
                    if ui.button("Spawn").clicked() {
                        let scale = ctx.screen_rect().max.x / test;
                        self.balls.push(Ball::new(
                            self.control_point * scale,
                            Vec2::ZERO,
                            self.new_col));
                    }
                    ui.color_edit_button_srgba(&mut self.new_col);
                });
            });
        });
    }
    fn position_picker(&mut self, ui: &mut Ui) -> f32 {
        let (response, painter) = ui.allocate_painter(Vec2::splat(ui.available_width()), Sense::hover());
        let to_screen = emath::RectTransform::from_to(Rect::from_min_size(Pos2::ZERO, response.rect.size()), response.rect);
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
            let stroke = ui.style().interact(&point_response).fg_stroke;

            Shape::circle_stroke(point_in_screen, control_point_radius, stroke)
        };
    
        painter.add(control_point_shape);

        ui.available_width()
    }
}

