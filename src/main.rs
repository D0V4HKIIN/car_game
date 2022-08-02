use raylib::prelude::*;

struct Car {
    body: Rectangle,
    acceleration: i32,
    speed: (i32, i32),
    angle: f32,
}

impl Car {
    fn draw(&self, mut d: RaylibDrawHandle) {
        //d.draw_text("car", self.pos.0, self.pos.1, self.width, Color::BLACK);
        d.draw_rectangle_pro(
            self.body,
            Vector2 {
                x: self.body.width / 2f32,
                y: self.body.width / 2f32,
            },
            self.angle,
            Color::RED,
        );
        d.draw_circle(self.body.x as i32, self.body.y as i32, 2f32, Color::BLUE);
    }
}

fn main() {
    let (mut rl, thread) = raylib::init().resizable().title("Hello, World").build();

    if let Ok(info) = get_monitor_info(0) {
        println!("{:#?}", info);
        rl.set_window_size(info.width / 2, info.height / 2);
        rl.set_window_min_size(info.width / 10, info.height / 10);
    }

    let mut default_car = Car {
        body: Rectangle {
            width: 45f32,
            height: 100f32,
            x: 100f32,
            y: 150f32,
        },
        acceleration: 10,
        speed: (0, 0),
        angle: 0f32,
    };

    let mut old_time = std::time::Instant::now();

    while !rl.window_should_close() {
        let new_time = std::time::Instant::now();
        let delta_time = new_time.duration_since(old_time).as_micros() as f32;
        old_time = new_time;

        let mut d = rl.begin_drawing(&thread);

        default_car.angle += 1f32 * delta_time / 10_000f32;

        d.clear_background(Color::GRAY);
        d.draw_text(
            &format!(
                "frame time: {:0>4} us\nfps: {:.1}",
                delta_time,
                1f32 / delta_time * 1_000_000f32
            ),
            12,
            12,
            20,
            Color::BLACK,
        );
        default_car.draw(d);
    }
}
