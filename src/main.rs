use raylib::prelude::*;

fn main() {
    let (mut rl, thread) = raylib::init().size(1000, 600).title("Zix").build();

    let mut x: f32 = 500.0;
    let mut y: f32 = 550.0;
    let speed: f32 = 0.03;
    while !rl.window_should_close() {
        rl.hide_cursor();

        if rl.is_key_down(KeyboardKey::KEY_UP) && y > 0.0 {
            y -= speed;
        }

        if rl.is_key_down(KeyboardKey::KEY_DOWN) && y < 550.0 {
            y += speed;
        }

        if rl.is_key_down(KeyboardKey::KEY_LEFT) && x > 0.0 {
            x -= speed;
        }

        if rl.is_key_down(KeyboardKey::KEY_RIGHT) && x < 950.0 {
            x += speed;
        }

        let mut d = rl.begin_drawing(&thread);
        d.clear_background(Color::BLACK);
        d.draw_text(
            &format!("X: {:.0}, Y: {:.0}", x, y),
            10,
            10,
            20,
            Color::HOTPINK,
        );

        d.draw_rectangle(x as i32, y as i32, 50, 50, Color::HOTPINK);
    }
}
