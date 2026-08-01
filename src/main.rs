use std::vec;

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
        active: true,
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

    let clrs: Vec<Color> = vec![
        Color::ORANGERED,
        Color::FLORALWHITE,
        Color::CRIMSON,
        Color::SILVER,
        Color::PINK,
        Color::TURQUOISE,
    ];

    let mut rng = rand::rng();
    let mut i: u16 = rng.random_range(4..16);
    let mut enemies: Vec<Enemy> = Vec::new();
    while i > 0 {
        rng = rand::rng();
        enemies.push(Enemy {
            width: rng.random_range(50..100) as i32,
            height: rng.random_range(20..80) as i32,
            x: rng.random_range(0..950) as f32,
            y: 0.0,
            speed: rng.random_range(0.001..0.005),
            color: clrs[rng.random_range(0..5)],
            active: true,
            mode: "normal".to_string(),
        });
        i -= 1;
    }

    while !rl.window_should_close() {
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

        if bullet.active {
            bullet.y -= bullet.speed;
            bullet.color = Color::RED;

            if bullet.y as i32 == 0 {
                bullet.color = Color::BLACK;
            }
            if bullet.y < 0.0 {
                bullet.active = false;
            }

            // TODO: fix when enemy should be killed
            // Kill Enemy
            for enemy in enemies.iter_mut() {
                if bullet.y as u32 == enemy.y as u32
                    && (bullet.x as u32 <= (enemy.x as u32 + enemy.width as u32)
                        && (bullet.x as u32 + enemy.width as u32)
                            >= (enemy.x as u32 + enemy.width as u32))
                {
                    enemy.color = Color::GREEN;
                    enemy.active = false;
                }
            }
        }

        for enemy in enemies.iter_mut() {
            if enemy.active {
                if enemy.y >= 550.0 {
                    enemy.active = false;
                    enemy.color = Color::BLACK;
                }
                enemy.y += enemy.speed;
            }
        }

        if player.active {
            d.clear_background(Color::BLACK);
            d.draw_text(
                &format!("X: {:.0}, Y: {:.0}", player.x, player.y),
                10,
                10,
                20,
                Color::LIMEGREEN,
            );
            enemies.retain(|enemy| enemy.active);
            println!("{:?}", enemies);

            // Draw Enemy
            for enemy in enemies.iter_mut() {
                d.draw_rectangle(
                    enemy.x as i32,
                    enemy.y as i32,
                    enemy.width,
                    enemy.height,
                    enemy.color,
                );
            }

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
        } else {
            d.clear_background(Color::RAYWHITE);
            d.clear_background(Color::BLACK);
            d.draw_text("GAME OVER", 10, 10, 40, Color::LIMEGREEN);
        }
    }
}
