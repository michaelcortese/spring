use macroquad::prelude::*;
use macroquad::ui::root_ui;

const DAMPING: f32 = 0.98;

#[derive(Default, Clone, Copy)]
struct Body {
    pos: Vec2,
    vel: Vec2,
    acc: Vec2,
    mass: f32,
}

impl Body {
    fn add_force(&mut self, force: Vec2) {
        // F = ma => a = F/m
        self.acc += force / self.mass;
    }

    fn update(&mut self) {
        self.vel += self.acc;
        self.vel *= DAMPING;
        self.pos += self.vel;
        self.acc = Vec2::default();
    }
}

#[macroquad::main("Spring")]
async fn main() {
    let mut k: f32 = 0.05;

    // F = -kx
    let origin = vec2(0.0, 50.0);
    let mut spring = Body {
        pos: vec2(0.0, 300.0),
        vel: Vec2::default(),
        acc: Vec2::default(),
        mass: 2.0,
    };
    let rest_length: f32 = 200.0;
    loop {
        clear_background(WHITE);

        root_ui().label(None, "Spring Constant");
        root_ui().slider(0, "", 0.01..1.0, &mut k);
        if is_mouse_button_pressed(MouseButton::Left) {
            let (mx, my) = mouse_position();
            // convert mouse to physics space
            let mouse_world = vec2(mx - screen_width() / 2.0, my);
            // direction from mass to mouse
            let to_mouse = mouse_world - spring.pos;
            spring.add_force(to_mouse * 0.5); // scale so it’s not insane
        }

        let mut dir = spring.pos - origin;
        let current_length = dir.length();
        dir = dir.normalize();
        let stretch = current_length - rest_length;

        let force = dir * (-k * stretch);
        spring.add_force(force);

        // gravity
        spring.add_force(vec2(0.0, 0.5 * spring.mass));
        spring.update();

        // draw

        draw_line(
            origin.x + screen_width() / 2.0,
            origin.y,
            spring.pos.x + screen_width() / 2.0,
            spring.pos.y,
            3.0,
            BLACK,
        );
        draw_circle(
            spring.pos.x + screen_width() / 2.0,
            spring.pos.y,
            10.0,
            BLACK,
        );

        next_frame().await
    }
}
