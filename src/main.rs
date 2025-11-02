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
    let mut k: f32 = 0.01;

    // F = -kx
    let origin = vec2(0.0, 50.0);
    let mut spring = Body {
        pos: vec2(0.0, 300.0),
        vel: Vec2::default(),
        acc: Vec2::default(),
        mass: 2.0,
    };
    let rest_length: f32 = 200.0;

    // input muts
    let mut x: String = "100.0".to_owned();
    let mut y: String = "-200".to_owned();

    loop {
        clear_background(WHITE);

        // UI
        {
            root_ui().label(None, "Spring Constant");
            root_ui().slider(0, "", 0.01..1.0, &mut k);
            root_ui().input_text(1, "-- x force", &mut x);
            root_ui().input_text(2, "-- y force", &mut y);

            if root_ui().button(None, "Apply forces") {
                let x = x.parse::<f32>().unwrap_or(0.0);
                let y = y.parse::<f32>().unwrap_or(0.0);
                spring.add_force(vec2(x, y));
            }
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

        // draw line from origin to body (sphere)

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
