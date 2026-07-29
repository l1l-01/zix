use rand::RngExt;
use raylib::prelude::*;

mod types;
use crate::types::{Bullet, Enemy, Player};

fn main() {
    let (mut rl, thread) = raylib::init().size(1000, 600).title("Zix").build();
    let mut player = Player {
        width: 50,
        height: 50,
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

    let mut rng = rand::rng();
    let mut enemy = Enemy {
        width: 60,
        height: 30,
        x: rng.random_range(0..950) as f32,
        y: 0.0,
        speed: 0.005,
        color: Color::PURPLE,
        active: true,
        mode: "normal".to_string(),
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

        let mut d: RaylibDrawHandle = rl.begin_drawing(&thread);
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

            // Kill Enemy
            if bullet.y as u32 == enemy.y as u32
                && (bullet.x as u32 <= (enemy.x as u32 + enemy.width as u32)
                    && (bullet.x as u32 + enemy.width as u32)
                        >= (enemy.x as u32 + enemy.width as u32))
            {
                println!(
                    "Enemy x: {}, Enemy y: {}, Bullet x: {}, Bullet y: {}",
                    enemy.x, enemy.y as u32, bullet.x, bullet.y as u32
                );

                enemy.color = Color::BLACK;
                enemy.active = false;
            }
        }

        if enemy.active {
            if enemy.y >= 550.0 {
                enemy.active = false;
                enemy.color = Color::BLACK;
            }
            enemy.y += enemy.speed;
        }

        // Draw Enemy
        d.draw_rectangle(
            enemy.x as i32,
            enemy.y as i32,
            enemy.width,
            enemy.height,
            enemy.color,
        );

        // Draw Bullet
        d.draw_rectangle(bullet.x as i32, bullet.y as i32, 10, 10, bullet.color);

        // Draw Player
        d.draw_rectangle(
            player.x as i32,
            player.y as i32,
            player.width,
            player.height,
            player.color,
        );
    }
}
