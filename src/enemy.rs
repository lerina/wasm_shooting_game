use crate::engine::{GAME_HEIGHT, GAME_WIDTH};

enum Direction {
    LEFT,
    RIGHT,
}

pub struct EnemyController {
    timer_til_next_move: f32,
    direction: Direction,
    pub enemies: Vec<Vec<Enemy>>,
}

impl EnemyController {
    pub fn new(level: i32) -> Self {
        EnemyController {
            timer_til_next_move: 0.0,
            direction: Direction::LEFT,
            enemies: spawn_ennemies(level),
        }
    }
    pub fn move_enemies(&mut self, width: f32, height: f32) {
        let mut mv_down = false;
        for row in &mut self.enemies {
            match self.direction {
                Direction::LEFT => {
                    for enemy in row {
                        if enemy.x <= 0.0 {
                            self.direction = Direction::RIGHT;
                            mv_down = true;
                        } else {
                            enemy.x -= enemy.speed;
                        }
                    }
                }
                Direction::RIGHT => {
                    for enemy in row {
                        if enemy.x >= GAME_WIDTH - enemy.width {
                            self.direction = Direction::LEFT;
                            mv_down = true;
                        } else {
                            enemy.x += enemy.speed;
                        }
                    }
                }
            } //^--match
        } //^-- for row
        if mv_down {
            self.move_down();
        }
    } //^--fn move_enemies

    pub fn has_reached_bottom(&self, max: f32) -> bool {
        let last_row = &self.enemies.last().unwrap();

        //    let enemy = last_row.last();
        match last_row.last() {
            Some(enemy) => {
                if enemy.y >= max - enemy.height {
                    return true;
                }
                return false;
            }
            _ => false,
        }
    }

    fn move_down(&mut self) {
        for row in &mut self.enemies {
            for enemy in row.iter_mut() {
                enemy.y += enemy.height * 0.3;
            }
        }
    } //^--fn move_down
    pub fn all_dead(&self) -> bool {    
        // As we level up we'll have variable numbers of rows
        // so check that each rows are empty    
        let max = self.enemies.len();
        let mut cnt = 0;
        for i in 1..=max {
            if self.enemies[max-i].len() == 0 { 
                cnt +=1; 
            }
        }
        
        max == cnt //all rows empty?
    }
} //^--impl

pub fn spawn_ennemies(level: i32) -> Vec<Vec<Enemy>> {
    vec![
        vec![
            Enemy::new(50.0, 20.0, "green".into(), 5),
            Enemy::new(150.0, 20.0, "green".into(), 5),
            Enemy::new(250.0, 20.0, "gold".into(), 6*level),
            Enemy::new(350.0, 20.0, "green".into(), 5),
            Enemy::new(450.0, 20.0, "green".into(), 5),
        ],
        vec![
            Enemy::new(50.0, 100.0, "green".into(), 2),
            Enemy::new(150.0, 100.0, "red".into(), 2),
            Enemy::new(250.0, 100.0, "yellow".into(), 7*level),
            Enemy::new(350.0, 100.0, "red".into(), 2),
            Enemy::new(450.0, 100.0, "green".into(),2),
        ],
        vec![
            Enemy::new(50.0, 180.0, "green".into(), 5),
            Enemy::new(150.0, 180.0, "gold".into(), 5),
            Enemy::new(250.0, 180.0, "blue".into(), 5*level),
            Enemy::new(350.0, 180.0, "gold".into(), 5),
            Enemy::new(450.0, 180.0, "green".into(),5),
        ],
    ]
} //^--fn spawn_ennemies

//------------------
pub struct Enemy {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
    pub speed: f32,
    color: String,
    pub health: i32,
}

impl Enemy {
    pub fn new(x: f32, y: f32, color: String, health: i32) -> Self {
        Enemy {
            x,
            y,
            width: 40.0,
            height: 40.0,
            speed: 0.7,
            color,
            health,
        }
    }

    pub fn take_damage(&mut self, damage: i32) {
        self.health -= damage;
    }
    pub fn color(&self) -> String {
        format!("{}", self.color)
    }
}
