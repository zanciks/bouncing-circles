use crate::{dot::Dot, ring::Ring};
use eframe::egui::{
    CentralPanel, Color32, Context, Pos2, Rect, Sense, Shape, Slider, Stroke, Ui, Vec2, Window,
};
use log::info;
use wasm_bindgen::prelude::*;

pub struct Simulation {
    paused: bool,
    simulation_area: Rect,

    dots: Vec<Dot>,
    dot_size: f32,
    gravity: f32,
    air_resistance: f32,

    rings: Vec<Ring>,
    ring_thickness: f32,

    control_dot: Dot,
    control_ring: Ring,
}

impl Default for Simulation {
    fn default() -> Simulation {
        Simulation {
            paused: true,
            simulation_area: Rect::ZERO,

            dots: vec![],
            dot_size: 15.0,
            gravity: 5.0,
            air_resistance: 0.05,

            rings: vec![],
            ring_thickness: 5.0,

            control_dot: Dot {
                position: Pos2::new(110.0, 110.0),
                ..Default::default()
            },
            control_ring: Ring::default(),
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

        if !self.paused {
            ctx.input(|i| self.physics_update(i.stable_dt));
            ctx.request_repaint()
        }

        CentralPanel::default().show(ctx, |ui| {
            self.draw_update(ui);
        });
    }
}

impl Simulation {
    fn draw_update(&self, ui: &mut Ui) {
        ui.painter().rect_filled(
            self.simulation_area,
            0.0,
            ui.style().visuals.extreme_bg_color,
        );

        for dot in &self.dots {
            ui.painter()
                .circle_filled(dot.position, self.dot_size, dot.color);
        }

        for ring in &self.rings {
            ui.painter().circle_stroke(
                ring.center,
                ring.radius,
                Stroke::new(self.ring_thickness, ring.color),
            );
        }
    }
    fn physics_update(&mut self, delta: f32) {
        let cloned_dots = self.dots.clone();

        for dot in &mut self.dots {
            for other_dot in &cloned_dots {
                if dot != other_dot {
                    let direction =
                        (dot.position.to_vec2() - other_dot.position.to_vec2()).normalized();
                    let distance = dot.position.distance(other_dot.position);

                    if distance <= 2.0 * self.dot_size && distance >= 0.5 * self.dot_size {
                        dot.velocity -= 2.0 * dot.velocity.dot(direction) * direction;
                    }
                }
            }

            for ring in &self.rings {
                let outer_radius = ring.radius + self.ring_thickness + self.dot_size;
                let inner_radius = ring.radius - self.dot_size;

                let distance = ring.center.distance(dot.position);
                let penetration_depth = outer_radius - distance;

                if distance >= inner_radius && distance <= outer_radius {
                    let normal = (dot.position.to_vec2() - ring.center.to_vec2()).normalized();
                    dot.velocity -= 2.0 * dot.velocity.dot(normal) * normal;

                    if penetration_depth > 0.0 {
                        dot.position = ring.center + normal * inner_radius;
                    }
                }
            }

            let dot_bottom = dot.position.y + self.dot_size;
            let dot_top = dot.position.y - self.dot_size;
            let dot_left = dot.position.x - self.dot_size;
            let dot_right = dot.position.x + self.dot_size;

            dot.velocity += Vec2::new(0.0, self.gravity * delta);
            dot.velocity *= (1.0 - self.air_resistance).powf(delta);
            dot.position += dot.velocity;

            if dot_bottom > self.simulation_area.max.y {
                dot.position.y = self.simulation_area.max.y - self.dot_size;
                dot.velocity.y *= -1.0;
            } else if dot_top < self.simulation_area.min.y {
                dot.position.y = self.simulation_area.min.y + self.dot_size;
                dot.velocity.y *= -1.0;
            }

            if dot_left < self.simulation_area.min.x {
                dot.position.x = self.simulation_area.min.x + self.dot_size;
                dot.velocity.x *= -1.0;
            } else if dot_right > self.simulation_area.max.x {
                dot.position.x = self.simulation_area.max.x - self.dot_size;
                dot.velocity.x *= -1.0;
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

            ui.add(Slider::new(&mut self.dot_size, 1.0..=25.0).text("Dot Size"));
            ui.add(Slider::new(&mut self.gravity, 1.0..=10.0).text("Gravity"));
            ui.add(Slider::new(&mut self.air_resistance, 0.0..=1.0).text("Air Resistance"));
            ui.add(Slider::new(&mut self.ring_thickness, 5.0..=25.0).text("Ring Thickness"));

            ui.shrink_width_to_current();

            ui.collapsing("Spawn new dot", |ui| {
                let size = self.move_control_dot(ui);
                ui.horizontal(|ui| {
                    if ui.button("Spawn").clicked() {
                        let scale = self.simulation_area.width() / size;
                        self.dots.push(Dot::new(
                            (self.control_dot.position * scale)
                                + self.simulation_area.left_top().to_vec2(),
                            Vec2::ZERO,
                            self.control_dot.color,
                        ));
                    }
                    ui.color_edit_button_srgba(&mut self.control_dot.color);
                });
            });

            ui.collapsing("Spawn new ring", |ui| {
                ui.add(
                    Slider::new(
                        &mut self.control_ring.radius,
                        self.dot_size..=(self.simulation_area.width() / 2.1),
                    )
                    .text("Ring Size"),
                );
                ui.horizontal(|ui| {
                    if ui.button("Spawn").clicked() {
                        self.rings.push(Ring::new(
                            self.simulation_area.center(),
                            self.control_ring.radius,
                            self.control_ring.color,
                        ))
                    };
                    ui.color_edit_button_srgba(&mut self.control_ring.color);
                });
            })
        });
    }
    fn move_control_dot(&mut self, ui: &mut Ui) -> f32 {
        let width = ui.available_width();
        let (response, painter) = ui.allocate_painter(Vec2::splat(width), Sense::click_and_drag());
        let to_screen = emath::RectTransform::from_to(
            Rect::from_min_size(Pos2::ZERO, response.rect.size()),
            response.rect,
        );
        let control_point_radius = 5.0;

        painter.rect_filled(response.rect, 0.0, ui.style().visuals.extreme_bg_color);

        let control_point_shape: Shape = {
            let size = Vec2::splat(2.0 * control_point_radius);

            let point_in_screen = to_screen.transform_pos(self.control_dot.position);
            let point_rect = Rect::from_center_size(point_in_screen, size);
            let point_id = response.id.with(0);
            let point_response = ui.interact(point_rect, point_id, Sense::drag());

            self.control_dot.position += point_response.drag_delta();
            self.control_dot.position = to_screen.from().clamp(self.control_dot.position);

            let point_in_screen = to_screen.transform_pos(self.control_dot.position);
            let mut stroke = Stroke::new(1.0, self.control_dot.color);

            if point_response.dragged() {
                stroke.color = stroke.color.lerp_to_gamma(Color32::WHITE, 0.6);
            } else if point_response.hovered() {
                stroke.color = stroke.color.lerp_to_gamma(Color32::WHITE, 0.4);
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
