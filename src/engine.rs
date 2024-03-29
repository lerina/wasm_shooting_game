use crate::browser;
use crate::enemy::{EnemyController, spawn_ennemies};
use crate::player::Player;
use crate::state;
use crate::view::web_renderer as renderer;
use crate::weapon::gun::BulletController;

pub const GAME_WIDTH: f32 = 600.0;
pub const GAME_HEIGHT: f32 = 600.0;
pub const CANVAS_NAME: &str = "canvas";
pub const PLAYER_STARTING_X: f32 = GAME_WIDTH / 2.0 - 25.0;
pub const PLAYER_STARTING_Y: f32 = GAME_HEIGHT - 55.0;

struct Point {
    x: f32,
    y: f32,
}

pub struct Game {
    player: Player,
    enemy_ctrl: EnemyController,
    pub renderer: renderer::Renderer,
    game_state: state::GameState,
    bullet_controller: BulletController,
    level: i32,
    score: i32,
    best: i32,
    shoot_pressed: bool,
}

impl Game {
    pub fn new() -> Self {
        Game {
            player: Player::new(PLAYER_STARTING_X, PLAYER_STARTING_Y),
            enemy_ctrl: EnemyController::new(1),
            renderer: renderer::Renderer::new(0.0, 0.0, GAME_WIDTH, GAME_HEIGHT, &CANVAS_NAME),
            game_state: state::GameState::Menu,
            bullet_controller: BulletController::new(),
            level: 1,
            score: 0,
            best: 0,
            shoot_pressed: false,
        }
    }

    pub fn update(&mut self, keystate: &mut browser::KeyState) {
        match self.game_state {
            state::GameState::Playing => {
                let mut velocity = Point { x: 0.0, y: 0.0 };
                if keystate.is_pressed("ArrowDown") {
                    velocity.y += 3.0;
                }

                if keystate.is_pressed("ArrowUp") {
                    velocity.y -= 3.0;
                }

                if keystate.is_pressed("ArrowRight") {
                    velocity.x += 3.0;
                }

                if keystate.is_pressed("ArrowLeft") {
                    velocity.x -= 3.0;
                }
        
                if keystate.is_pressed("Space") {
                    self.shoot_pressed = true;
                }

                //---------------------

                if self.enemy_ctrl.all_dead() {
                    self.level_up();
                    self.game_state = state::GameState::Levelup;
                }
                if self
                    .enemy_ctrl
                    .has_reached_bottom(GAME_HEIGHT - self.player.height - 57.0)
                    //if DEV 
                    //.has_reached_bottom(self.player.y)
                {
                    self.game_state = state::GameState::Gameover;
                }else {
                    self.enemy_ctrl.move_enemies(GAME_WIDTH, GAME_HEIGHT);

                self.move_player(velocity);
                self.handle_shoot();
                self.handle_enemy();
                self.bullet_controller.remove_dead_bullets();
                self.handle_best_score(); 
               }

            }, //^--Playing
            state::GameState::Menu => {
                if keystate.is_pressed("Space") {
                    self.game_state = state::GameState::Ready;
                    keystate.drain();
                    self.reset();
                }
            },//^-- reset
            state::GameState::Gameover => {
                if keystate.is_pressed("KeyY") {
                    self.game_state = state::GameState::Menu;
                    keystate.drain();
                }
                if keystate.is_pressed("KeyN") {
                    self.game_state = state::GameState::Topscore;
                    keystate.drain();
                }
                //self.handle_gameover();
            }, //^-- Gameover
            state::GameState::Ready => {
                if keystate.is_pressed("Space") {
                    self.bullet_controller.drain();
                    keystate.drain();
                    self.game_state = state::GameState::Playing;
                }
            }, //^--Ready
            state::GameState::Topscore => {
                if keystate.is_pressed("Space") {
                    self.game_state = state::GameState::Menu;
                    keystate.drain();
                }
            }, //^--Topscore
            state::GameState::Levelup => {
                if keystate.is_pressed("Space") {
                    self.game_state = state::GameState::Ready;
                    keystate.drain();
                }
            }, //^--Levelup
        } //^-- match self.game_state
    } //^--fn update()

    fn move_player(&mut self, velocity: Point) {
        self.player.x += velocity.x;
        self.player.y += velocity.y;
    }

    fn handle_shoot(&mut self) {
        if self.shoot_pressed {
            self.bullet_controller
                .shoot(self.player.x + self.player.width / 2.0, self.player.y, 5.0, 1, 7.0);

            self.shoot_pressed = false;
        }
    } //^--fn handle_shoot

    fn handle_enemy(&mut self) {
        //self.enemy_ctrl.
        for row in &mut self.enemy_ctrl.enemies {
            for enemy in row {
                if self.bullet_controller.collide_with(enemy) {
                    self.score += 1;
                }
            }
        }

        self.rm_dead_enemies();
    } //^--fn handle_enemy
    fn rm_dead_enemies(&mut self) {
        for row in &mut self.enemy_ctrl.enemies {
            row.retain(|enemy| enemy.health > 0);
        }
    }
    fn level_up(&mut self) {
        self.level +=1;
        self.enemy_ctrl.enemies = spawn_ennemies(self.level);
    }
    fn reset(&mut self) {
        self.level =1;
        self.score =0;
        self.enemy_ctrl.enemies = spawn_ennemies(self.level);
        self.player.x = PLAYER_STARTING_X;
        self.player.y = PLAYER_STARTING_Y;
    }
    fn handle_best_score(&mut self) {
        if self.best < self.score {
            self.best = self.score
        }
    }
    pub fn draw(&mut self) {
        self.renderer.clear_canvas();
        match self.game_state {
            state::GameState::Playing => {
                self.draw_enemies();
                self.draw_bullets();
                self.show_game_info();
                self.draw_player();
            },
            state::GameState::Ready => {
                self.show_game_info();
                self.draw_ready(); 
            },
            state::GameState::Gameover => {
                self.draw_gameover();
            },
            state::GameState::Topscore => {
                self.draw_top_score();
            },
            state::GameState::Levelup => {
                self.draw_level_up();
            },
            state::GameState::Menu => {
                self.draw_menu();
            },
        } //^--match
    }
    fn draw_menu(&self) {
        self.renderer.draw_menu();
    }
    fn draw_ready(&self) {
        self.renderer.draw_ready(self.level);
    }
    fn draw_gameover(&self) {
        self.renderer.draw_gameover();
    }

    fn draw_top_score(&self) {
        self.renderer.draw_top_score(self.best, self.score);
    }
    fn show_game_info(&self) {
       self.renderer.text_score(self.score);
       self.renderer.text_level(self.level);
    }
    fn draw_level_up(&self) {
        self.renderer.draw_level_up(self.level);
    }
    fn draw_player(&self) {
        self.renderer.draw_player(&self.player);
    }

    fn draw_enemies(&self) {
        for enemy_row in &self.enemy_ctrl.enemies {
            for enemy in enemy_row {
                self.renderer.draw_enemy(&enemy);
            }
        }
    } //^-- draw_enemies

    fn draw_bullets(&mut self) {
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
            self.renderer.draw_bullet(&bullet);
        }
        for bullet in &mut self.bullet_controller.bullets {
            bullet.y -= bullet.speed();
        }
    }
}
