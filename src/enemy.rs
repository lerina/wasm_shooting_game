pub struct Enemy {
    x: f32,
    y: f32,
    width: f32,
    height: f32,
    speed: f32,
    color: String,
    health: i32,
}

impl Enemy {
    pub fn new(x: f32, y: f32, color: String, health: i32) -> Self {
        Enemy {
            x,
            y,
            width: 50.0,
            height: 50.0,
            speed: 1.5,
            color,
            health,
        }
    }
    pub fn x(&self) -> f32 {
        self.x
    }
    pub fn set_x(&mut self, x: f32) {
        self.x = x;
    }
    pub fn y(&self) -> f32 {
        self.y
    }
    pub fn set_y(&mut self, y: f32) {
        self.y = y;
    }
    pub fn width(&self) -> f32 {
        self.width
    }
    pub fn height(&self) -> f32 {
        self.height
    }
    pub fn health(&self) -> i32 {
        self.health
    }
    pub fn take_damage(&mut self, damage: i32) {
        self.health -= damage;
    }
    pub fn color(&self) -> String {
        format!("{}", self.color)
    }
}
