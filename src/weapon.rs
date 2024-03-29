pub mod gun {

    use crate::enemy::Enemy;

    pub struct BulletController {
        pub bullets: Vec<Bullet>,
        timer_till_next_bullet: f32,
    }

    impl BulletController {
        pub fn new() -> BulletController {
            BulletController {
                bullets: Vec::new(),
                timer_till_next_bullet: 0.0,
            }
        }
        pub fn remove_dead_bullets(&mut self) {
            self.bullets.retain(|bullet| !bullet.remove)
        }
        pub fn drain(&mut self) {
            self.bullets.drain(..);
        }
        pub fn is_bullet_off_screen(bullet: &Bullet) -> bool {
            bullet.y <= bullet.height * 2.0
        }

        pub fn shoot(&mut self, x: f32, y: f32, speed: f32, damage: i32, delay: f32) {
            if self.timer_till_next_bullet <= 0.0 {
                self.bullets.push(Bullet::new(x, y, speed, damage));

                self.timer_till_next_bullet = delay;
            }

            self.timer_till_next_bullet -= 1.0;
        } //^-- shoot
        pub fn collide_with(&mut self, enemy: &mut Enemy) -> bool {
            for bullet in &mut self.bullets {
                if bullet.collide_with(enemy) {
                    bullet.remove();
                    return true;
                }
            }

            false
        } //^--fn collide_with
    } //^--impl BulletController

    //#[derive(Copy, Clone)]
    pub struct Bullet {
        pub x: f32,
        pub y: f32,
        pub width: f32,
        pub height: f32,
        speed: f32,
        delay: f32,
        damage: i32,
        pub remove: bool,
    }

    impl Bullet {
        pub fn new(x: f32, y: f32, speed: f32, damage: i32) -> Self {
            Bullet {
                x,
                y,
                width: 3.0,
                height: 5.0,
                speed,
                delay: 7.0,
                damage,
                remove: false,
            }
        }

        pub fn speed(&self) -> f32 {
            self.speed
        }
        pub fn delay(&self) -> f32 {
            self.delay
        }
        pub fn damage(&self) -> i32 {
            self.damage
        }
        pub fn collide_with(&self, other: &mut Enemy) -> bool {
            if self.x < other.x + other.width
                && self.x + self.width > other.x
                && self.y < other.y + other.height
                && self.y + self.height > other.y
            {
                other.take_damage(self.damage);
                return true;
            }
            false
        } //^--fn collide_with
        pub fn remove(&mut self) {
            self.remove = true;
        }
    } //^--impl Bullet
} //^--mod laser
