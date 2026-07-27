use raylib::prelude::*;

struct Bullet {
    x: f32,
    y: f32,
    speed: f32,
    active: bool,
    color: Color,
}

fn main() {
    let (mut rl, thread) = raylib::init().size(1000, 600).title("Zix").build();

    let mut player_x: f32 = 500.0;
    let mut player_y: f32 = 550.0;
    let player_speed: f32 = 0.03;
    let mut bullet = Bullet {
        x: player_x + 20.0,
        y: player_y - 10.0,
        speed: 0.05,
        active: false,
        color: Color::BLACK,
    };

    while !rl.window_should_close() {
        rl.hide_cursor();

        if rl.is_key_down(KeyboardKey::KEY_UP) && player_y > 0.0 {
            player_y -= player_speed;
        }

        if rl.is_key_down(KeyboardKey::KEY_DOWN) && player_y < 550.0 {
            player_y += player_speed;
        }

        if rl.is_key_down(KeyboardKey::KEY_LEFT) && player_x > 0.0 {
            player_x -= player_speed;
        }

        if rl.is_key_down(KeyboardKey::KEY_RIGHT) && player_x < 950.0 {
            player_x += player_speed;
        }

        if rl.is_key_pressed(KeyboardKey::KEY_SPACE) && !bullet.active {
            bullet.active = true;
            bullet.x = player_x + 20.0;
            bullet.y = player_y - 10.0;
        }

        let mut d = rl.begin_drawing(&thread);
        d.clear_background(Color::BLACK);
        d.draw_text(
            &format!("X: {:.0}, Y: {:.0}", player_x, player_y),
            10,
            10,
            20,
            Color::LIMEGREEN,
        );

        if bullet.active {
            bullet.y -= bullet.speed;
            bullet.color = Color::RED;

            if bullet.y as i32 == 0 {
                bullet.color = Color::BLACK;
            }
            if bullet.y < 0.0 {
                bullet.active = false;
            }
        }

        d.draw_rectangle(player_x as i32, player_y as i32, 50, 50, Color::LIMEGREEN);
        d.draw_rectangle(bullet.x as i32, bullet.y as i32, 10, 10, bullet.color);
    }
}
