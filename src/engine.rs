use crate::browser;
use crate::enemy::Enemy;
use crate::player::Player;
use crate::weapon::gun::{Bullet, BulletController};

const GAME_WIDTH: f32 = 600.0;
const GAME_HEIGHT: f32 = 600.0;

struct Point {
    x: f32,
    y: f32,
}

pub struct Game {
    x: f32,
    y: f32,
    width: f32,
    height: f32,
    pub time_frame: f64,
    pub player: Player,
    enemies: Vec<Enemy>,
    pub bullet_controller: BulletController,
    shoot_pressed: bool,
    cnv: web_sys::HtmlCanvasElement,
    ctx: web_sys::CanvasRenderingContext2d,
}

impl Game {
    pub fn new() -> Self {
        let enemies = vec![
            Enemy::new(50.0, 20.0, "green".into(), 5),
            Enemy::new(150.0, 20.0, "red".into(), 5),
            Enemy::new(250.0, 20.0, "gold".into(), 2),
            Enemy::new(350.0, 20.0, "green".into(), 2),
            Enemy::new(450.0, 20.0, "gold".into(), 10),
            Enemy::new(50.0, 100.0, "green".into(), 5),
            Enemy::new(150.0, 100.0, "red".into(), 5),
            Enemy::new(250.0, 100.0, "gold".into(), 2),
            Enemy::new(350.0, 100.0, "green".into(), 2),
            Enemy::new(450.0, 100.0, "gold".into(), 20),
        ];

        Game {
            x: 0.0,
            y: 0.0,
            width: GAME_WIDTH,
            height: GAME_HEIGHT,
            time_frame: 0.0,
            player: Player::new(GAME_WIDTH / 2.0 - 25.0, GAME_HEIGHT - 55.0),
            enemies,
            bullet_controller: BulletController::new(),
            shoot_pressed: false,
            cnv: browser::canvas(),
            ctx: browser::context(),
        }
    }
    pub fn init(&mut self) {
        //self.laser.push(Bullet::new(GAME_WIDTH / 2.0 - 5.0, GAME_HEIGHT - 58.0));
        let _ = self.cnv.focus();
        self.set_common_style();
    }
    pub fn update(&mut self, keystate: &browser::KeyState) {
        let mut velocity = Point { x: 0.0, y: 0.0 };
        if keystate.is_pressed("ArrowDown") {
            velocity.y += 3.0;
            //log!("ArrowDown");
        }

        if keystate.is_pressed("ArrowUp") {
            velocity.y -= 3.0;
            //log!("ArrowUp");
        }

        if keystate.is_pressed("ArrowRight") {
            velocity.x += 3.0;
            //log!("ArrowRight");
        }

        if keystate.is_pressed("ArrowLeft") {
            velocity.x -= 3.0;
            //log!("ArrowLeft");
        }
        if keystate.is_pressed("Space") {
            self.shoot_pressed = true;
            //log!("Spacebar");
        }

        self.player.set_x(self.player.x() + velocity.x);
        self.player.set_y(self.player.y() + velocity.y);
        self.handle_shoot();
        self.bullet_controller.remove_dead_bullets();
        self.handle_enemy();
    }

    fn handle_shoot(&mut self) {
        if self.shoot_pressed {
            self.bullet_controller.shoot(
                self.player.x() + self.player.width() / 2.0,
                self.player.y(),
                5.0,
                1,
                7.0,
            );

            self.shoot_pressed = false;
        }
        //self.bullet_controller.shoot();
    }
    pub fn handle_enemy(&mut self) {
        for enemy in &mut self.enemies {
            if self.bullet_controller.collide_with(&enemy) {
                enemy.take_damage(1); // losing ability to change damage based on bullet
            }
        }
        self.enemies.retain(|enemy| enemy.health() > 0);
    }
    pub fn set_common_style(&self) {
        self.ctx.set_shadow_color("#d53");
        self.ctx.set_shadow_blur(20.0);
        self.ctx.set_line_join("bevel");
        self.ctx.set_line_width(5.0);
    }
    pub fn clear_canvas(&self) {
        self.ctx.set_fill_style(&"rgb(0,0,0)".into());
        self.ctx
            .clear_rect(self.x.into(), self.y.into(), self.width.into(), self.height.into());
        self.ctx
            .fill_rect(self.x.into(), self.y.into(), self.width.into(), self.height.into());
    }
    pub fn draw(&mut self) {
        self.clear_canvas();
        //        for bullet in &self.laser {
        //            if !bullet.remove {
        //                self.draw_laser(&bullet);
        //            }
        //        }
        self.draw_bullets();
        self.draw_player();
        self.draw_enemies();
    }
    pub fn draw_player(&self) {
        self.ctx.set_stroke_style(&"yellow".into()); // &JsValue::from_str("yellow")
        self.ctx.set_fill_style(&"black".into()); // into know to use JsValue::from_str
        self.ctx.stroke_rect(
            self.player.x().into(),
            self.player.y().into(),
            self.player.width().into(),
            self.player.height().into(),
        );
        self.ctx.fill_rect(
            self.player.x().into(),
            self.player.y().into(),
            self.player.width().into(),
            self.player.height().into(),
        );
    }
    pub fn draw_bullets(&mut self) {
        /*
        SOURCE: https://users.rust-lang.org/t/finding-and-removing-an-element-in-a-vec/42166
        Sol 1.
        if let Some(index) = some_vec.iter().position(|value| *value == 10) {
            some_vec.swap_remove(index);
        }

        Sol 2.
        let mut some_vec = vec![0, 10, 20, 30];
        some_vec.retain(|value| *value != 10);

        Note that in case there are several elements with value 10,
        this will remove all of them, which might or might not be what you want.
        */
        self.bullet_controller
            .bullets
            .retain(|bullet| !BulletController::is_bullet_off_screen(&bullet));

        for bullet in &self.bullet_controller.bullets {
            self.draw_bullet(&bullet);
        }
        for bullet in &mut self.bullet_controller.bullets {
            let y = bullet.y();
            bullet.set_y(y - bullet.speed());
        }
    }

    pub fn draw_bullet(&self, bullet: &Bullet) {
        self.ctx.set_stroke_style(&"red".into()); // &JsValue::from_str("yellow")
        self.ctx.set_fill_style(&"grey".into()); // into know to use JsValue::from_str
        self.ctx.stroke_rect(
            bullet.x().into(),
            bullet.y().into(),
            bullet.width().into(),
            bullet.height().into(),
        );
        self.ctx.fill_rect(
            bullet.x().into(),
            bullet.y().into(),
            bullet.width().into(),
            bullet.height().into(),
        );
    }
    fn draw_enemies(&self) {
        for enemy in &self.enemies {
            self.draw_enemy(&enemy);
        }
    }
    fn draw_enemy(&self, enemy: &Enemy) {
        self.ctx.set_fill_style(&enemy.color().into());
        if enemy.health() > 1 {
            self.ctx.set_stroke_style(&"white".into());
        } else {
            self.ctx.set_stroke_style(&enemy.color().into());
        }

        //DEBUG: web_sys::console::log_1(&format!("draw me: x:{} y:{}", enemy.x(), enemy.y()).into());

        self.ctx.fill_rect(
            enemy.x().into(),
            enemy.y().into(),
            enemy.width().into(),
            enemy.height().into(),
        );
        self.ctx.stroke_rect(
            enemy.x().into(),
            enemy.y().into(),
            enemy.width().into(),
            enemy.height().into(),
        );
        //Draw Text
        let health = format!("{}", enemy.health());
        self.ctx.set_fill_style(&"black".into());
        self.ctx.set_font(&"25px Arial");
        let _ = self.ctx.fill_text(
            &health,
            (enemy.x() + enemy.width() / 3.5).into(),
            (enemy.y() + enemy.height() / 1.5).into(),
        );
    }
} //^--impl Game
