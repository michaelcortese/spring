use macroquad::prelude::*;
use macroquad::ui::root_ui;

const DAMPING: f32 = 0.995;
const G: f32 = 9.81;

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

    fn update(&mut self, delta_time: f32) {
        self.vel += self.acc * delta_time;
        self.vel *= DAMPING.powf(delta_time * 60.0);
        self.pos += self.vel * delta_time;
        self.acc = Vec2::default();
    }
}

#[derive(Clone, Copy)]
struct Spring {
    origin: Vec2,
    rest_length: f32,
}

fn calc_spring_force(spring: Spring, body: Body, k: f32) -> Vec2 {
    let mut dir = body.pos - spring.origin;
    let current_length = dir.length();
    dir = dir.normalize();
    let stretch = current_length - spring.rest_length;

    dir * (-k * stretch)
}

#[macroquad::main("Spring")]
async fn main() {
    let mut k: f32 = 1.5;

    // F = -kx
    // let spring1 = vec2(0.0, 50.0);
    // let spring2 = vec2(0.0, screen_height()-50.0);

    let springs = [
        Spring {
            origin: vec2(0.0, 50.0),
            rest_length: 200.0,
        },
        Spring {
            origin: vec2(100.0, screen_height() - 50.0),
            rest_length: 200.0,
        },
    ];

    let mut body = Body {
        pos: vec2(0.0, 300.0),
        vel: Vec2::default(),
        acc: Vec2::default(),
        mass: 2.0,
    };
    //let rest_length: f32 = 200.0;

    // input muts
    let mut x: String = "30.0".to_owned();
    let mut y: String = "-50.0".to_owned();
    let mut gravity = false;

    loop {
        clear_background(WHITE);
        let dt = get_frame_time() * 5.0;
        // UI
        {
            root_ui().label(None, "Spring Constant");
            root_ui().slider(0, "", 0.01..5.0, &mut k);
            root_ui().input_text(1, "-- x force", &mut x);
            root_ui().input_text(2, "-- y force", &mut y);
            root_ui().checkbox(3, "Gravity", &mut gravity);
            if root_ui().button(None, "Apply forces") {
                let x = x.parse::<f32>().unwrap_or(0.0);
                let y = y.parse::<f32>().unwrap_or(0.0);
                body.add_force(vec2(x, y) * 1000.0);
            }
            root_ui().label(None, &format!("FPS: {}", get_fps()));
        }

        springs.iter().for_each(|&spring| {
            let force = calc_spring_force(spring, body, k);
            body.add_force(force);
        });

        // gravity
        if gravity {
            body.add_force(vec2(0.0, G * body.mass));
        }
        body.update(dt);

        // draw line from origin to body (sphere)

        draw_line(
            springs[0].origin.x + screen_width() / 2.0,
            springs[0].origin.y,
            body.pos.x + screen_width() / 2.0,
            body.pos.y,
            3.0,
            BLACK,
        );
        draw_line(
            springs[1].origin.x + screen_width() / 2.0,
            springs[1].origin.y,
            body.pos.x + screen_width() / 2.0,
            body.pos.y,
            3.0,
            BLACK,
        );
        draw_circle(body.pos.x + screen_width() / 2.0, body.pos.y, 10.0, BLACK);

        next_frame().await
    }
}
