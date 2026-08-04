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
        mode: "normal".to_string(),
        color: Color::LIMEGREEN,
    };

    let mut bullet = Bullet {
        x: player.x + 20.0,
        y: player.y - 10.0,
        speed: 0.05,
        active: false,
        color: Color::BLACK,
    };

    let mut score: i16 = 0;

    let mut rng = rand::rng();
    let mut i: u16 = rng.random_range(10..15);
    let mut enemies: Vec<Enemy> = Vec::new();
    let mut minions: Vec<Enemy> = Vec::new();

    while i > 0 {
        let speed = rng.random_range(1..5) as f32 / 1000.0;
        let clr: Color;
        let mode: String;
        let damage: i32;

        match speed {
            0.004 => {
                clr = Color::ORANGE;
                mode = "speed".to_string();
                damage = 1;
            }
            0.003 => {
                clr = Color::CYAN;
                mode = "stretch".to_string();
                damage = 2;
            }
            0.002 => {
                clr = Color::SALMON;
                mode = "camelion".to_string();
                damage = 5;
            }
            0.001 => {
                clr = Color::RED;
                mode = "multiplier".to_string();
                damage = 10;
            }
            _ => {
                clr = Color::SILVER;
                mode = "none".to_string();
                damage = 1;
            }
        };
        enemies.push(Enemy {
            width: rng.random_range(50..100) as i32,
            height: rng.random_range(20..50) as i32,
            x: rng.random_range(0..950) as f32,
            y: 0.0,
            speed: speed,
            color: clr,
            active: true,
            mode: mode,
            damage: damage,
        });
        i -= 1;
    }

    while !rl.window_should_close() {
        for value in (5..=200).step_by(5) {
            if score == value {
                // Return player speed into normal
                player.speed = 0.03;
                player.color = Color::LIMEGREEN;
                player.mode = "normal".to_string();
                bullet.speed = 0.05;
            } else {
                // Boost player speed
                bullet.speed = 0.08;
                player.speed = 0.05;
                player.color = Color::YELLOW;
                player.mode = "speed".to_string();
            }
            break;
        }

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
            bullet.color = player.color;

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
                    enemy.active = false;
                    if enemy.mode == "multiplier".to_string() {
                        let mut n: u8 = rng.random_range(0..4);
                        while n > 0 {
                            minions.push(Enemy {
                                width: 50,
                                height: 20,
                                x: rng.random_range(0..950) as f32,
                                y: 0.0,
                                speed: 0.01,
                                color: Color::RED,
                                active: true,
                                mode: "minion".to_string(),
                                damage: 2,
                            });
                            n -= 1;
                        }
                    }
                    score += 1;
                }
            }
        }

        let mut len = minions.len();
        while len > 0 {
            let m = minions.remove(len - 1);
            enemies.push(m);
            len -= 1;
        }

        for enemy in enemies.iter_mut() {
            if enemy.active {
                if enemy.y >= 550.0 {
                    enemy.active = false;
                    enemy.color = Color::BLACK;
                    score -= enemy.damage as i16;

                    if score < 0 {
                        player.active = false;
                    }
                }
                enemy.y += enemy.speed;
            }
        }

        if enemies.len() == 0 && player.active {
            i = rng.random_range(10..20);

            while i > 0 {
                let speed = rng.random_range(1..5) as f32 / 1000.0;
                let clr: Color;
                let mode: String;
                let damage: i32;

                match speed {
                    0.004 => {
                        clr = Color::ORANGE;
                        mode = "speed".to_string();
                        damage = 1;
                    }
                    0.003 => {
                        clr = Color::CYAN;
                        mode = "stretch".to_string();
                        damage = 2;
                    }
                    0.002 => {
                        clr = Color::SALMON;
                        mode = "camelion".to_string();
                        damage = 5;
                    }
                    0.001 => {
                        clr = Color::RED;
                        mode = "multiplier".to_string();
                        damage = 10;
                    }
                    _ => {
                        clr = Color::SILVER;
                        mode = "none".to_string();
                        damage = 1;
                    }
                };
                enemies.push(Enemy {
                    width: rng.random_range(50..100) as i32,
                    height: rng.random_range(20..50) as i32,
                    x: rng.random_range(0..950) as f32,
                    y: 0.0,
                    speed: speed,
                    color: clr,
                    active: true,
                    mode: mode,
                    damage: damage,
                });
                i -= 1;
            }
        }

        if player.active {
            d.clear_background(Color::BLACK);

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

            d.draw_text(
                &format!("Score: {}, Mode: {}", score, player.mode),
                10,
                10,
                20,
                player.color,
            );
            enemies.retain(|enemy| enemy.active);
        } else {
            d.clear_background(Color::BLACK);
            d.draw_text("GAME OVER", 10, 10, 40, Color::RED);
            d.draw_text("Press 'P' to play!", 10, 60, 20, Color::RED);
        }
    }
}
