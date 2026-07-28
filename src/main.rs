use raylib::prelude::*;

mod types;
use crate::types::{Bullet, Player};

fn main() {
    let (mut rl, thread) = raylib::init().size(1000, 600).title("Zix").build();
    let mut player = Player {
        x: 500.0,
        y: 550.0,
        speed: 0.03,
        mode: "bullet".to_string(),
        color: Color::LIMEGREEN,
    };

    let mut bullet = Bullet {
        x: player.x + 20.0,
        y: player.y - 10.0,
        speed: 0.05,
        active: false,
        color: Color::BLACK,
    };

    while !rl.window_should_close() {
        rl.hide_cursor();

        if rl.is_key_down(KeyboardKey::KEY_UP) && player.y > 0.0 {
            player.y -= player.speed;
        }

        if rl.is_key_down(KeyboardKey::KEY_DOWN) && player.y < 550.0 {
            player.y += player.speed;
        }

        if rl.is_key_down(KeyboardKey::KEY_LEFT) && player.x > 0.0 {
            player.x -= player.speed;
        }

        if rl.is_key_down(KeyboardKey::KEY_RIGHT) && player.x < 950.0 {
            player.x += player.speed;
        }

        if rl.is_key_pressed(KeyboardKey::KEY_SPACE) && !bullet.active {
            bullet.active = true;
            bullet.x = player.x + 20.0;
            bullet.y = player.y - 10.0;
        }

        let mut d = rl.begin_drawing(&thread);
        d.clear_background(Color::BLACK);
        d.draw_text(
            &format!("X: {:.0}, Y: {:.0}", player.x, player.y),
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

        d.draw_rectangle(bullet.x as i32, bullet.y as i32, 10, 10, bullet.color);
        d.draw_rectangle(player.x as i32, player.y as i32, 50, 50, player.color);
    }
}
